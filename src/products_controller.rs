
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::products_functions::*;
use crate::establish_connection;
use crate::db::products_functions::Params;

#[get("/?<sort>&<brandId>&<typeId>&<name>&<pageIndex>&<pageSize>")]
pub fn get_all_products
(
    sort: Option<String>,
    #[allow(non_snake_case)]
    brandId: Option<i32>,
    #[allow(non_snake_case)]
    typeId: Option<i32>,
    name: Option<String>,
    #[allow(non_snake_case)]
    pageIndex: Option<i64>,
    #[allow(non_snake_case)]
    pageSize: Option<i64>,
) -> Json<PaginatedResult> {
    let conn = establish_connection();
    Json(get_products_with_params(&conn, Params{
        sort: sort.clone(),
        brand_id: brandId.clone(),
        type_id: typeId.clone(),
        name: name.clone(),
        page_index: pageIndex.clone(),
        page_size: pageSize.clone()
    }))
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