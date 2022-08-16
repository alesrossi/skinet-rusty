use diesel::{PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::schema::product_brands::dsl::product_brands;
use crate::db::schema::product_types::dsl::product_types;
use crate::db::schema::products::dsl::*;

#[derive(Debug)]
pub struct Params {
    pub name: Option<String>,
    pub sort_by: Option<String>,
}

// pub fn get_products(connection: &PgConnection) -> Vec<Product> {
//     products
//         .load::<Product>(connection)
//         .expect("Error loading posts")
//
// }

pub fn get_products_with_params(connection: &PgConnection,params: Params) -> Result<Json<Vec<Product>>, NotFound<String>> {
    let results;
    if let Some(d_name) = params.name {
        results = products.filter(name.like(d_name)).load::<Product>(connection);
    } else {
        results = products.load::<Product>(connection);
    }
    match results {
        Ok(product) => {
            if product.is_empty() {
                Err(NotFound(String::from("Product Not Found")))
            } else {
                Ok(Json(product))
            }

        },
        Err(_) => Err(NotFound(String::from("Product Not Found"))),
    }
}

pub fn get_product(product_id: i32, connection: &PgConnection) -> Result<Json<Product>, NotFound<String>> {
    match products.find(product_id).first(connection) {
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