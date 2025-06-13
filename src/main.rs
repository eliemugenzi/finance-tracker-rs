use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod config;
mod middleware;
mod utils;
mod modules;

use utils::constants::{api, server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT")
        .unwrap_or_else(|_| server::PORT.to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::logging::Logging)
            .service(
                web::scope(api::API_PREFIX)
                    .configure(modules::users::routes::init)
                    .configure(modules::transactions::routes::init)
            )
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}