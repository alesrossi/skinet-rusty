
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::products_functions::*;
use crate::establish_connection;
use crate::db::products_functions::Params;

#[get("/?<name>")]
pub fn get_all_products(name: Option<String>) -> Result<Json<Vec<Product>>, NotFound<String>> {
    let conn = establish_connection();
    get_products_with_params(&conn, Params{name: name.clone(), sort_by: None })
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