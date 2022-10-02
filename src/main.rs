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
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_cors::Cors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let actix_address = env!("ACTIX_ADDRESS");
    let actix_port = env!("ACTIX_PORT").parse::<u16>().unwrap();
    env_logger::init();
    info!("Initialising server on {}, port {}", actix_address, actix_port);
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();



    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::permissive()
            )


            .wrap(middleware::NormalizePath::new(TrailingSlash::Always))
            .wrap(Logger::default())
            .service(web::scope("/api/products").configure(products_routes))
            .service(web::scope("/api/basket").configure(basket_routes))
            .service(web::scope("/api/account").configure(account_routes))
            .service(web::scope("/api/orders").configure(orders_routes))
            .service(Files::new("/","./dist").index_file("index.html"))

    })
        .bind_openssl((actix_address, actix_port), builder)?
        .run()
        .await
}

// Missing features
// Foreign keys and unique, Test/Debug
// Testing
// Rename columns in prod
// Caching
// Add more features
// OkAPi
// Errors controller maybe
// upgrade diesel
// enable payments and stripe
// Github actions