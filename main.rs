mod config;
mod models;
mod handlers;
mod middleware;
mod database;
mod utils;
mod errors;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;

use config::Config;
use database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Connect to database
    log::info!("Attempting to connect to SQLite database: {}", config.database_url);
    let database = match Database::new(&config.database_url).await {
        Ok(db) => {
            log::info!("Successfully connected to SQLite database");
            // Run migrations
            if let Err(e) = db.migrate().await {
                log::error!("Failed to run database migrations: {}", e);
                std::process::exit(1);
            }
            db
        }
        Err(e) => {
            log::error!("Failed to connect to SQLite database: {}", e);
            log::error!("Database file: {}", config.database_url);
            std::process::exit(1);
        }
    };
    
    let host = config.host.clone();
    let port = config.port;
    let frontend_url = config.frontend_url.clone();
    
    log::info!("Starting server at http://{}:{}", host, port);
    log::info!("Frontend URL configured: {}", frontend_url);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_origin("http://localhost:3000") // Primary frontend port
            .allowed_origin("http://localhost:5173") // Alternative frontend port
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .supports_credentials()
            .max_age(3600);
            
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(database.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(handlers::auth::register))
                            .route("/login", web::post().to(handlers::auth::login))
                            .route("/me", web::get().to(handlers::auth::get_current_user))
                    )
            )
            .route("/health", web::get().to(handlers::health::health_check))
    })
    .bind((host, port))?
    .run()
    .await
}
