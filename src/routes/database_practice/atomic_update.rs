use axum::extract::State;
use axum::{extract::Path, Json, http::StatusCode};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait,Set};
use sea_orm::{DatabaseConnection, prelude::DateTimeWithTimeZone};
use serde::{Serialize, Deserialize};
use crate::database::tasks::Entity as Task;

use crate::database::tasks;

#[derive(Serialize,Deserialize)]
pub struct RequestTask{
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

pub async fn atomic_update(
    Path(task_id):Path<i32>,
    State(db):State<DatabaseConnection>,
    Json(task):Json<RequestTask>
)->Result<(),StatusCode>{
    let update_task = tasks::ActiveModel{
        id: Set(task.id),
        priority: Set(task.priority),
        title: Set(task.title),
        completed_at: Set(task.completed_at),
        description: Set(task.description),
        deleted_at: Set(task.deleted_at),
        user_id: Set(task.user_id),
        is_default: Set(task.is_default),
    };

    Task::update(update_task)
    .filter(tasks::Column::Id.eq(task_id))
    .exec(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}