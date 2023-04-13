use axum::Json;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct MirrorJson{
    message:String
}

#[derive(Serialize)]
pub struct ResponseJson{
    message:String,
    author:String,
}

pub async fn mirror_body_json(Json(body):Json<MirrorJson>)->Json<ResponseJson>{
    Json(ResponseJson { message: body.message, author: "author 1".to_owned() })
}