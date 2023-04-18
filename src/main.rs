use dotenvy::dotenv;
use std::env;
use rust_axum_sample::run;


#[tokio::main]
async fn main() { 
   
   dotenv().ok();
   let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

   run(&db_url).await;
}

