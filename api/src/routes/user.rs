use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use rocket::{http::Status, response::status::BadRequest, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{authentication::Claims, LoginResponse, MyState};

#[derive(FromRow)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    data: serde_json::Value,
    completed_tasks: Vec<i32>,
    quiz_results: serde_json::Value,
}
#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}
#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    data: serde_json::Value,
    completed_tasks: Vec<i32>,
}
impl UserResponse {
    pub fn from_user(user: User) -> Self {
        UserResponse {
            username: user.username,
            data: user.data,
            completed_tasks: user.completed_tasks,
        }
    }
}

#[get("/data")]
pub async fn get_user(
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

#[post("/edit_tree", data = "<data>")]
pub async fn edit_tree(
    state: &State<MyState>,
    claims: Claims,
    data: Json<serde_json::Value>,
) -> Result<Status, BadRequest<String>> {
    let _user: Option<User> = sqlx::query_as("UPDATE users SET data = $1 WHERE username = $2")
        .bind(data.0)
        .bind(claims.name)
        .fetch_optional(&state.0)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Status::Ok)
}

#[post("/edit_quiz", data = "<data>")]
pub async fn edit_quiz(
    state: &State<MyState>,
    claims: Claims,
    data: Json<serde_json::Value>,
) -> Result<Status, BadRequest<String>> {
    let _user: Option<User> =
        sqlx::query_as("UPDATE users SET quiz_results = $1 WHERE username = $2")
            .bind(data.0)
            .bind(claims.name)
            .fetch_optional(&state.0)
            .await
            .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Status::Ok)
}

#[post("/complete_task", data = "<task_id>")]
pub async fn complete_task(
    state: &State<MyState>,
    claims: Claims,
    task_id: Json<i32>,
) -> Result<Status, BadRequest<String>> {
    let _user: User =
        sqlx::query_as("UPDATE users SET array_append(completed_tasks, $1) WHERE username = $2")
            .bind(task_id.0)
            .bind(claims.name)
            .fetch_one(&state.0)
            .await
            .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Status::Ok)
}

#[post("/register", data = "<data>")]
pub async fn register(
    state: &State<MyState>,
    data: Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, BadRequest<String>> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(data.0.password.as_bytes(), &salt)
        .map_err(|err| BadRequest(Some(err.to_string())))?
        .to_string();

    let user = Json(User {
        username: data.0.username,
        password_hash,
        data: serde_json::Value::Null,
        completed_tasks: vec![],
        quiz_results: serde_json::Value::Null,
    });
    let user:User = sqlx::query_as(
        "INSERT INTO users(username, password_hash, data, completed_tasks, quiz_results) VALUES ($1,$2,$3, $4, $5) RETURNING id, username, password_hash, data, completed_tasks, quiz_results",
    )
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(&user.data)
    .bind(&user.completed_tasks)
    .bind(&user.quiz_results)
    .fetch_one(&state.0)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    let claim = Claims::from_name(&user.username);

    let response = LoginResponse {
        token: claim
            .into_token()
            .map_err(|error| BadRequest(Some(error.1)))?,
        username: user.username,
    };
    Ok(Json(response))
}
