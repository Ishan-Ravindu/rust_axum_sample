use axum::{extract::Path, Extension, Json, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::tasks::Entity as Task;

#[derive(Serialize)]
pub struct ResponseTask{
    id:i32,
    title:String,
    priority:Option<String>,
    description:Option<String>
}

pub async fn get_one_task(Path(task_id):Path<i32>,Extension(db):Extension<DatabaseConnection>)->Result<Json<ResponseTask>,StatusCode>{

    let task = Task::find_by_id(task_id).one(&db).await.unwrap();

    if let Some(task)=task{
        Ok(
            Json(
                ResponseTask { id: task.id, title: task.title, priority: task.priority, description: task.description }
            )
        )
    }else{
        Err(StatusCode::NOT_FOUND)
    }

}

pub async fn get_all_task(Extension(db):Extension<DatabaseConnection>)->Result<Json<Vec<ResponseTask>>,StatusCode>{
    let task = Task::find()
        .all(&db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task|ResponseTask{
            id:db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();
    Ok(Json(task))
}