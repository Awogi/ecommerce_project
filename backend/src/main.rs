#![recursion_limit = "1024"]

mod config;
mod controller;
mod dto;
mod middleware;
mod error;
mod model;
mod routes;
mod schema;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Logger};
// use dotenvy crate (already in Cargo.toml)
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

use config::establish_connection;
use middleware::auth::AuthMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Database connection
    let db_pool = establish_connection();
    
    // Default to binding on all interfaces so the server is reachable on LAN.
    // If HOST is set in the environment we respect it (useful for CI or docker).
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", host, port);

    // Print both LAN and localhost addresses for developer convenience.
    let localhost_url = format!("http://127.0.0.1:{}", port);
    // Try to detect a LAN IPv4 address for display; fallback to host if HOST provided
    let lan_ip = local_ipaddress::get().unwrap_or(host.clone());
    let lan_url = format!("http://{}:{}", lan_ip, port);

    println!("🚀 Server starting");
    println!("   Local: {}", localhost_url);
    println!("   LAN:   {}", lan_url);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() 
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let auth_middleware = Arc::new(AuthMiddleware);
            
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(auth_middleware))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(routes::init_routes)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
