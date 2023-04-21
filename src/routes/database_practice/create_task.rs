use axum::{Json, extract::State};
use sea_orm::{DatabaseConnection,Set, ActiveModelTrait};
use serde::Deserialize;
use crate::database::tasks;


#[derive(Deserialize)]
pub struct RequestTask{
    title:String,
    priority:Option<String>,
    description:Option<String>
}

pub async fn create_task(
    State(db):State<DatabaseConnection>,
    Json(request_task):Json<RequestTask>
){
   
    let new_task = tasks::ActiveModel{
        title:Set(request_task.title),
        priority:Set(request_task.priority),
        description:Set(request_task.description),
        ..Default::default()
    };

    let result = new_task.save(&db).await.unwrap();

    dbg!(result);
}