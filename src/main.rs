#[macro_use]
extern crate diesel;
#[macro_use] extern crate rocket;

use crate::db::{create_poduct, establish_connection};
use crate::products_controller::get_all;
mod db;
mod products_controller;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/products", routes![
            get_all
        ])

}
