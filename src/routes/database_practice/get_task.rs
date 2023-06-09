use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, EntityTrait, Condition, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::database::tasks::{Entity as Task, self};

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db):State<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Task::find_by_id(task_id).one(&db).await.unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct GetTaskQueryParams {
    priority: Option<String>,
}

pub async fn get_all_task(
    State(db):State<DatabaseConnection>,
    Query(query_param):Query<GetTaskQueryParams>
)-> Result<Json<Vec<ResponseTask>>, StatusCode> {

    let mut priority_filter = Condition::all();

    if let Some(priority) = query_param.priority{
        priority_filter = if priority.is_empty(){
            priority_filter.add(tasks::Column::Priority.is_null())
        }else{
            priority_filter.add(tasks::Column::Priority.eq(priority))
        }
    }

    let task = Task::find()
        .filter(priority_filter)
        .all(&db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();
    Ok(Json(task))
}
