use axum::Json;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug,Deserialize)]
pub struct UserRequest{
    user_name:Option<String>,
    password:String,
}

pub async fn validate_with_serde(Json(user):Json<UserRequest>){
    dbg!(user);
}