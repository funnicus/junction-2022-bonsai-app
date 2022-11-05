use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use rocket::{http::Status, response::status::BadRequest, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{authentication::Claims, routes::user::UserResponse, MyState};

struct Task {
    title: String,
    description: String,
}

#[get("/tasks")]
pub async fn get_tasks(
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
/*
#[post("/task/add_task", data = "<data>")]
pub async fn add_task(
    state: &State<MyState>,
    data: Json<serde_json::Value>,
) -> Result<Status, BadRequest<String>> {
    let _user: Task = sqlx::query_as("UPDATE users SET data = $1 WHERE username = $2")
        .bind(data.0)
        .bind(claims.name)
        .fetch_one(&state.0)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Status::Ok)
}
*/
