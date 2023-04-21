mod simple_route_practice;
mod database_practice;

use axum::{
    http::Method,
    routing::{get, post,put, patch, delete},
    Router, middleware, extract::FromRef,
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};
use simple_route_practice::hello_world::hello_world;
use simple_route_practice::middleware_message::middleware_message;
use simple_route_practice::mirror_body_json::mirror_body_json;
use simple_route_practice::mirror_body_string::mirror_body_string;
use simple_route_practice::mirror_custom_header::mirror_custom_header;
use simple_route_practice::mirror_user_agent::mirror_user_agent;
use simple_route_practice::path_variables::{hard_coded_path, path_variables};
use simple_route_practice::query_params::query_params;
use simple_route_practice::read_middleware_custom_header::read_middleware_custom_header;
use simple_route_practice::set_middleware_custom_header::set_middleware_custom_header;
use simple_route_practice::always_errors::always_errors;
use simple_route_practice::return_201::return_201;
use simple_route_practice::get_json::get_json;
use simple_route_practice::validate_with_serde::validate_with_serde;
//database practice routes
use database_practice::create_task::create_task;
use database_practice::custom_json_extractor::custom_json_extractor;
use database_practice::get_task::get_all_task;
use database_practice::get_task::get_one_task;
use database_practice::atomic_update::atomic_update;
use database_practice::partial_update::partial_update;
use database_practice::delete_task::delete_task;

#[derive(Clone,FromRef)]
pub struct AppState {
    message: String,
    database:DatabaseConnection
}

pub fn create_routes(db:DatabaseConnection) -> Router {

    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any);

    let app_state = AppState {
        message: "hello from shared data".to_owned(),
        database:db
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/50", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .route("/always_errors", get(always_errors))
        .route("/return_201", post(return_201))
        .route("/get_json",get(get_json))
        .route("/validate_with_serde", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/task", post(create_task))
        .route("/task", get(get_all_task))
        .route("/task/:task_id", get(get_one_task))
        .route("/atomic_update/:task_id", put(atomic_update))
        .route("/partial_update/:task_id", patch(partial_update))
        .route("/task/:task_id", delete(delete_task))
        .with_state(app_state)
}
