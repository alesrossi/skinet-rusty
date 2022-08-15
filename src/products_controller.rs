use rocket::http::{ContentType, Header};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::products_functions::*;
use crate::establish_connection;

#[get("/")]
pub fn get_all_products() -> Json<Vec<Product>> {
    let conn = establish_connection();
    Json(get_products(&conn))
}

#[get("/<id>")]
pub fn get_one(id: i32,) -> Result<Json<Product>, NotFound<String>> {
    let conn = establish_connection();
    get_product(id, &conn)
}

#[get("/brands")]
pub fn get_all_brands() -> Json<Vec<ProductBrand>> {
    let conn = establish_connection();
    Json(get_brands(&conn))
}

#[get("/types")]
pub fn get_all_types() -> Json<Vec<ProductType>> {
    let conn = establish_connection();
    Json(get_types(&conn))
}