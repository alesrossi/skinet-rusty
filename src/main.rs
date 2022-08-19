#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;
use rocket::fs::{FileServer, relative};
use rocket::http::Method;
use rocket::Request;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use crate::db::{establish_connection};
use crate::products_controller::*;
use crate::basket_controller::*;
mod db;
mod products_controller;
mod basket_controller;
mod responders;

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
    env_logger::init();
    debug!("Starting up");
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api/products",
               routes![
            get_all_products, get_one, get_all_brands, get_all_types,

        ])
        .mount("/api/basket",
               routes![
            get_basket_from_id, create_new_basket, delete_basket_from_id

        ])
        .register("/api/basket", catchers![malformed])
        .attach(cors.to_cors().unwrap())


}

#[catch(422)]
fn malformed(req: &Request) -> String {
    debug!("{req:?}");
    String::from("wtf")
}