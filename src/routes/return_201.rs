use axum::{response::{Response, IntoResponse}, http::StatusCode};

pub async fn return_201()->Response{
    let body = "this is body form return_201 route";
    (
        StatusCode::CREATED,
        body
    ).into_response()
}