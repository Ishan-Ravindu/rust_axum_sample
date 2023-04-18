use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data{
    name:String,
    age:i32,
    email:String
}

pub async fn get_json()->Json<Data>{
    let data =Data{
        name:"ishan".to_owned(),
        age:22,
        email:"test@gmail.com".to_owned()
    };

    Json(data)
}