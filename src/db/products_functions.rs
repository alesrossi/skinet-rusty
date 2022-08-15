use diesel::{PgConnection, Queryable, QueryDsl, RunQueryDsl};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::schema::product_brands::dsl::product_brands;
use crate::db::schema::product_types::dsl::product_types;
use crate::db::schema::products::dsl::products;

pub fn get_products(connection: &PgConnection) -> Vec<Product> {
    products
        .load::<Product>(connection)
        .expect("Error loading posts")


}

pub fn get_product(id: i32, connection: &PgConnection) -> Result<Json<Product>, NotFound<String>> {
    match products.find(id).first(connection) {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(NotFound(String::from("Product Not Found"))),
    }
}

pub fn get_brands(connection: &PgConnection) -> Vec<ProductBrand> {
    product_brands
        .load::<ProductBrand>(connection)
        .expect("Error loading posts")


}

pub fn get_types(connection: &PgConnection) -> Vec<ProductType> {
    product_types
        .load::<ProductType>(connection)
        .expect("Error loading posts")


}