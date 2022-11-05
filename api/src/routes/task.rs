use rocket::{http::Status, response::status::BadRequest, serde::json::Json, State};
use serde::Serialize;
use sqlx::FromRow;

use crate::MyState;

#[derive(Serialize, FromRow)]
pub struct Task {
    title: String,
    description: String,
    category: String,
}

#[get("/get_tasks")]
pub async fn get_tasks(state: &State<MyState>) -> Result<Json<Vec<Task>>, BadRequest<String>> {
    let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM tasks")
        .fetch_all(&state.0)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(tasks))
}

#[post("/add_task", data = "<data>")]
pub async fn add_task(
    state: &State<MyState>,
    data: Json<serde_json::Value>,
) -> Result<Status, BadRequest<String>> {
    let _user: Task = sqlx::query_as(
        "INSERT INTO tasks(title, description, category) VALUES ($1,$2,$3) RETURNING title, description, category",
    )
    .bind(data.0.get("title").unwrap().as_str())
    .bind(data.0.get("description").unwrap().as_str())
    .bind(data.0.get("category").unwrap().as_str())
    .fetch_one(&state.0)
    .await
    .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Status::Ok)
}
