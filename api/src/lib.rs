#[macro_use]
extern crate rocket;

use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use shuttle_service::error::CustomError;
use sqlx::{Executor, FromRow, PgPool};

struct MyState(PgPool);

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    pub id: i32,
    pub username: String,
}

#[get("/")]
fn index() -> &'static str {
    "Bonsai"
}

#[get("/create_user")]
async fn create_user(state: &State<MyState>) -> Result<Json<User>, BadRequest<String>> {
    let user = Json(User {
        id: 1,
        username: "test".to_string(),
    });
    let user = sqlx::query_as("INSERT INTO users(username) VALUES ($1) RETURNING id, username")
        .bind(&user.username)
        .fetch_one(&state.0)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(user))
}
#[get("/users/<id>")]
async fn get_user(state: &State<MyState>, id: &str) -> Result<Json<User>, BadRequest<String>> {
    let user = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id.parse::<i32>().unwrap())
        .fetch_one(&state.0)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(user))
}

#[shuttle_service::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_service::ShuttleRocket {

    // For testing purposes we reinit database on deployment
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let state = MyState(pool);

    Ok(rocket::build()
        .manage(state)
        .mount("/", routes![index, create_user, get_user]))
}
