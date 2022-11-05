mod authentication;

#[macro_use]
extern crate rocket;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use authentication::Claims;
use rocket::response::status::{BadRequest, Unauthorized};
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
    pub password_hash: String,
    // data: serde_json::Value,
}
#[derive(Serialize)]
struct UserResponse {
    pub id: i32,
    pub username: String,
}
impl UserResponse {
    pub fn from_user(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
        }
    }
}
#[derive(Serialize, Deserialize)]
struct Data {
    r#type: String,
    time: i32,
    children: Vec<Data>,
    taskId: String,
    angle: Option<i32>,
    length: Option<i32>,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
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
        .map_err(|e| Unauthorized(Some(e.to_string())))?;

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
        };

        Ok(Json(response))
    } else {
        Err(Unauthorized(Some("Incorrect password".to_string())))
    }
}

#[get("/create_user")]
async fn create_user(state: &State<MyState>) -> Result<Json<UserResponse>, BadRequest<String>> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(b"test", &salt)
        .map_err(|err| BadRequest(Some(err.to_string())))?
        .to_string();

    let user = Json(User {
        id: 1,
        username: "test".to_string(),
        password_hash,
        /*   data: json!(Data {
            r#type: "root".to_string(),
            time: 1231212321,
            children: vec![],
            taskId: 1.to_string(),
            angle: None,
            length: None,
        }),*/
    });
    let user:User = sqlx::query_as(
        "INSERT INTO users(username, password_hash) VALUES ($1,$2) RETURNING id, username, password_hash",
    )
    .bind(&user.username)
    .bind(&user.password_hash)
    .fetch_one(&state.0)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(UserResponse::from_user(user)))
}

#[get("/user")]
async fn get_user(
    state: &State<MyState>,
    claims: Claims,
) -> Result<Json<UserResponse>, BadRequest<String>> {
    let user = sqlx::query_as("SELECT * FROM users WHERE username = $1")
        .bind(claims.name)
        .fetch_one(&state.0)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(UserResponse::from_user(user)))
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
        .mount("/", routes![index, create_user, get_user, login]))
}
