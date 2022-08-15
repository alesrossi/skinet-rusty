#[macro_use]
extern crate diesel;
#[macro_use] extern crate rocket;

use crate::db::{establish_connection};
use crate::products_controller::*;
mod db;
mod products_controller;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/products", routes![
            get_all_products, get_one, get_all_brands, get_all_types
        ])

}
