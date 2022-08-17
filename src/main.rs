#[macro_use]
extern crate diesel;
#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use crate::db::{establish_connection};
use crate::products_controller::*;
mod db;
mod products_controller;

#[launch]
fn rocket() -> _ {

    let allowed_origins = AllowedOrigins::some_exact(&["https://localhost:4200"]);
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api/products",
               routes![
            get_all_products, get_one, get_all_brands, get_all_types,

        ]).attach(cors.to_cors().unwrap())


}
