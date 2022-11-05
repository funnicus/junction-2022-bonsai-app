mod authentication;
mod cors;
pub mod routes;

#[macro_use]
extern crate rocket;

use crate::routes::task::{add_task, get_tasks};
use crate::routes::user::{complete_task, edit_quiz, edit_tree, get_user, register};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use authentication::Claims;
use cors::Cors;
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket::State;
use routes::user::User;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct MyState(PgPool);

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    username: String,
}

#[get("/")]
fn index() -> &'static str {
    "Bonsai"
}

#[post("/login", data = "<login>")]
async fn login(
    state: &State<MyState>,
    login: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Unauthorized<String>> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE username = $1")
        .bind(&login.username)
        .fetch_one(&state.0)
        .await
        .map_err(|_e| Unauthorized(Some("No such user".to_string())))?;

    let argon2 = Argon2::default();

    if argon2
        .verify_password(
            login.password.as_bytes(),
            &PasswordHash::new(&user.password_hash).unwrap(),
        )
        .is_ok()
    {
        let claim = Claims::from_name(&login.username);
        let response = LoginResponse {
            token: claim
                .into_token()
                .map_err(|error| Unauthorized(Some(error.1)))?,
            username: login.username.clone(),
        };

        Ok(Json(response))
    } else {
        Err(Unauthorized(Some("Incorrect password".to_string())))
    }
}

#[shuttle_service::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_service::ShuttleRocket {
    // For testing purposes we reinit database on deployment
    /*pool.execute(include_str!("../schema.sql"))
    .await
    .map_err(CustomError::new)?;*/
    let state = MyState(pool);

    Ok(rocket::build()
        .manage(state)
        .attach(Cors)
        .mount("/", routes![index, login, register])
        .mount(
            "/user",
            routes![get_user, complete_task, edit_tree, edit_quiz],
        )
        .mount("/tasks", routes![get_tasks, add_task]))
}
