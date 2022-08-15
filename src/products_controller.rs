use rocket::http::{ContentType, Header};
use rocket::serde::json::Json;
use crate::db::models::Product;
use crate::db::products_functions::*;
use crate::establish_connection;




#[get("/")]
pub fn get_all() -> Json<Vec<Product>> {
    let conn = establish_connection();
    Json(get_products(&conn))
}
