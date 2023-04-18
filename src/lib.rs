use routes::create_routes;
use sea_orm::{DatabaseConnection,Database};

mod routes;

pub async fn run(database_uri:&str){

    let _db: DatabaseConnection = Database::connect(database_uri)
        .await
        .expect("Database connection failed");

    let app = create_routes();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}