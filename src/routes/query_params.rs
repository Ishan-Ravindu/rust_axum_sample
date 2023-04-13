use axum::{extract::Query, Json};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct QueryParams{
    name:String,
    id:i32
}

pub async fn query_params(Query(query): Query<QueryParams>)->Json<QueryParams>{
    Json(query)
}