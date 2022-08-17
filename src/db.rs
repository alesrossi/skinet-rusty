pub mod schema;
pub mod models;
pub mod products_functions;
pub mod paginate;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;


pub fn establish_connection() -> PgConnection {
    let database_url = env!("DATABASE_URL").to_string();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



// pub fn create_poduct<'a>(conn: &PgConnection, name: &'a str, cost: &'a i32) -> Product {
//     use schema::products;
//
//     let new_post = NewProduct {
//         name,
//         cost,
//     };
//
//     diesel::insert_into(products::table)
//         .values(&new_post)
//         .get_result(conn)
//         .expect("Error saving new product")
// }