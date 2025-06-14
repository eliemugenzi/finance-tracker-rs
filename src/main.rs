use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use env_logger::Env;
use log::LevelFilter;

use finance_tracker::{
    modules::users::routes as user_routes,
    modules::transactions::routes as transaction_routes,
    utils::not_found,
    utils::constants::api::API_PREFIX,
    AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Configure logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .format_module_path(false)
        .format_target(false)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app_state = web::Data::new(AppState { db: pool });

    log::info!("🚀 Server starting at http://localhost:{}{}", port, API_PREFIX);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::new(
                "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"
            ))
            .app_data(app_state.clone())
            .service(
                web::scope(API_PREFIX)
                    .configure(user_routes::init)
                    .configure(transaction_routes::init)
            )
            .default_service(web::route().to(not_found::not_found))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}