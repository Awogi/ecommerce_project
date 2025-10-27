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
use actix_files::Files;
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
    // Parse port and prepare base port
    let base_port: u16 = port.parse().unwrap_or(8080);

    // Try binding to the configured port; if it's already in use, try the next ports up to
    // a small limit. If a non-AddrInUse error occurs, return it immediately.
    // Allow configuring how many ports to try via PORT_TRIES env var (useful for dev).
    let max_tries: u16 = match env::var("PORT_TRIES") {
        Ok(s) => s.parse().unwrap_or(10u16),
        Err(_) => 10u16,
    };

    // Ensure static directory exists so actix_files doesn't log errors when serving.
    let static_dir = "./static";
    if !std::path::Path::new(static_dir).exists() {
        match std::fs::create_dir_all(static_dir) {
            Ok(_) => println!("Created missing static dir: {}", static_dir),
            Err(e) => eprintln!("Warning: failed to create static dir '{}': {}", static_dir, e),
        }
    }

    // If the frontend's static assets are present in a sibling folder (../frontend/static),
    // copy any missing files into the backend static dir so images and other assets are
    // available when the backend serves the site. This helps local development where
    // images live in the frontend crate's `static/` folder.
    let frontend_static = "../frontend/static";
    if std::path::Path::new(frontend_static).exists() {
        match std::fs::read_dir(frontend_static) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let src = entry.path();
                    if src.is_file() {
                        if let Some(file_name) = src.file_name() {
                            let dst = std::path::Path::new(static_dir).join(file_name);
                            if !dst.exists() {
                                match std::fs::copy(&src, &dst) {
                                    Ok(_) => println!("Copied asset to backend static: {:?}", dst),
                                    Err(e) => eprintln!("Failed to copy asset {:?}: {}", src, e),
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed to read frontend static dir '{}': {}", frontend_static, e),
        }
    }
    let mut last_err: Option<std::io::Error> = None;
    for i in 0..max_tries {
        let try_port = base_port.saturating_add(i);
        let try_addr = format!("{}:{}", host, try_port);
        // Build a fresh server factory for every bind attempt so ownership isn't moved across tries.
        let db_pool_for_srv = db_pool.clone();
        let srv_factory = HttpServer::new(move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600);

            let auth_middleware = Arc::new(AuthMiddleware);

                App::new()
                .app_data(web::Data::new(db_pool_for_srv.clone()))
                .app_data(web::Data::new(auth_middleware))
                .wrap(cors)
                .wrap(Logger::default())
                // Mount API under /api so frontend can be served from root and call same-origin /api/*
                .service(web::scope("/api").configure(routes::init_routes))
                // Serve files under /static from the ./static folder so that image URLs like
                // /static/white-shirt-male.jpg map to ./static/white-shirt-male.jpg correctly.
                // This avoids the double-"static" path issue when the frontend references
                // /static/<file> but the Files root is mounted at '/'.
                .service(Files::new("/static", "./static").use_last_modified(true))
                // Serve static frontend files from ./static (index.html + wasm/assets) at root
                .service(Files::new("/", "./static").index_file("index.html"))
        });

        match srv_factory.bind(&try_addr) {
            Ok(bound_srv) => {
                // Successful bind, print addresses and run
                let localhost_url = format!("http://127.0.0.1:{}", try_port);
                let lan_ip = local_ipaddress::get().unwrap_or(host.clone());
                let lan_url = format!("http://{}:{}", lan_ip, try_port);

                println!("🚀 Server starting");
                println!("   Local: {}", localhost_url);
                println!("   LAN:   {}", lan_url);

                return bound_srv.run().await;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AddrInUse {
                    eprintln!("Port {} is in use, trying next port...", try_port);
                    last_err = Some(e);
                    continue;
                } else {
                    // Unexpected error; return it
                    return Err(e);
                }
            }
        }
    }

    // If we get here, we couldn't bind any ports in the range. Return the last error.
    if let Some(e) = last_err {
        Err(e)
    } else {
        // This shouldn't happen, but provide a generic error
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to bind server to any port"))
    }
}
