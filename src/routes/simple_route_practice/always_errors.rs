use axum::http::StatusCode;

pub async fn always_errors()->StatusCode{
    StatusCode::IM_A_TEAPOT
}