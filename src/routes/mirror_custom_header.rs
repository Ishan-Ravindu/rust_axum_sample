use axum::http::header::HeaderMap;

pub async fn mirror_custom_header(headers:HeaderMap)->String{
    headers.get("custom_header").unwrap().to_str().unwrap().to_owned()

}