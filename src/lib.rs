use routes::create_routes;
use sea_orm::{DatabaseConnection,Database};

mod routes;
mod database;

pub async fn run(database_uri:&str){

    let db: DatabaseConnection = Database::connect(database_uri)
        .await
        .expect("Database connection failed");

    let app = create_routes(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}