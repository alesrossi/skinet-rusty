
pub mod schema;
pub mod models;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::db::models::{NewProduct, Product};
use crate::db::schema::products::dsl::products;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn read_products(connection: &PgConnection,) {

    let results = products
        .limit(5)
        .load::<Product>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for product in results {
        println!("{}", product.name);
        println!("----------\n");
        println!("{}", product.cost);
    }
}

pub fn create_poduct<'a>(conn: &PgConnection, name: &'a str, cost: &'a i32) -> Product {
    use schema::products;

    let new_post = NewProduct {
        name,
        cost,
    };

    diesel::insert_into(products::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new product")
}