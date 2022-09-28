#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate core;

use log::info;
use actix_web::*;
use crate::controllers::products_controller::*;
use crate::controllers::basket_controller::*;
use crate::controllers::identity_controller::*;
use crate::controllers::orders_controller::*;
use actix_web::middleware::{Logger, TrailingSlash};
use actix_files::Files;
mod db;
mod controllers;
mod jwt;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let actix_address = env!("ACTIX_ADDRESS");
    let actix_port = env!("ACTIX_PORT").parse::<u16>().unwrap();
    env_logger::init();
    info!("Initialising server on {}, port {}", actix_address, actix_port);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::new(TrailingSlash::Always))
            .wrap(Logger::default())
            .service(web::scope("/api/products").configure(products_routes))
            .service(web::scope("/api/basket").configure(basket_routes))
            .service(web::scope("/api/account").configure(account_routes))
            .service(web::scope("/api/orders").configure(orders_routes))
            .service(Files::new("/", "./static").index_file("index.html"))

    })
        .bind((actix_address, actix_port))?
        .run()
        .await
}

// FIX PAGINATION
// Missing features
// Async
// Error handling/Refactoring
// TLS/Images
// Foreign keys and unique, Test/Debug
// Testing
// Rename columns in prod
// Caching
// Add more features
// Serve static images
// OkAPi