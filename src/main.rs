#[macro_use]
extern crate diesel;

use crate::db::{create_poduct, establish_connection, read_products};

mod db;

fn main() {
    let connection = establish_connection();
    // create_poduct(&connection, "Batteria", &12);
    read_products(&connection);
}
