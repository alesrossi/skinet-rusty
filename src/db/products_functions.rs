use diesel::{PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods, ExpressionMethods};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::schema::product_brands::dsl::product_brands;
use crate::db::schema::product_types::dsl::product_types;
use crate::db::schema::products;
use crate::db::paginate::LoadPaginated;
use crate::{filter, sort_by};
use serde::{Serialize};

#[derive(Debug)]
pub struct Params {
    pub sort: Option<String>,
    pub brand_id: Option<i32>,
    pub type_id: Option<i32>,
    pub name: Option<String>,
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResult {
    page_index: i64,
    page_size: i64,
    count: usize,
    data: Vec<Product>
}

pub fn get_product(product_id: i32, connection: &PgConnection) -> Result<Json<Product>, NotFound<String>> {
    match products::table.find(product_id).first(connection) {
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

pub fn get_products_with_params(connection: &PgConnection,params: Params) -> PaginatedResult {
    let mut query = products::table.into_boxed();

    // filtering
    query = filter!(query,
           (products::name, @like, params.name),
           (products::productbrand, @ge, params.brand_id),
           (products::productbrand, @le, params.brand_id),
           (products::producttype, @ge, params.type_id),
           (products::producttype, @le, params.type_id)
    );

    // sorting
    query = sort_by!(query, params.sort,
            ("id", products::id),
            ("name", products::name),
            ("brand", products::productbrand),
            ("type", products::producttype),
            ("price", products::price)
    );

    // result
    let result = query
        .load_with_pagination(connection, params.page_index, params.page_size).unwrap();

    PaginatedResult {
        page_index: params.page_index.unwrap_or_else(|| 1),
        page_size: result.2,
        count: result.0.len(),
        data: result.0
    }
}