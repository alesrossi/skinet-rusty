use rocket::serde::json::Json;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::products_functions::*;
use crate::db::products_functions::Params;
use crate::responders::ApiError;
use log::{error};
use crate::db::utils::DbError;

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
) -> Result<Json<PaginatedResult>, ApiError> {
    match get_products_with_params(Params{
        sort: sort.clone(),
        brand_id: brandId.clone(),
        type_id: typeId.clone(),
        name: name.clone(),
        page_index: pageIndex.clone(),
        page_size: pageSize.clone()
    }) {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            error!("{err:?}");
            Err(ApiError::InternalServer(String::from("Internal Server Error")))
        }
    }
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<Product>, ApiError> {
    match get_product(id) {
        Ok(product) => Ok(Json(product)),
        Err(error) => {
            error!("{error:?}");
            match error.current_context() {
                DbError::NotFoundError => Err(ApiError::NotFound(format!("Product '{id}' not found"))),
                _ => Err(ApiError::InternalServer(String::from("Internal Server Error")))
            }
        }
    }
}

#[get("/brands")]
pub fn get_all_brands() -> Result<Json<Vec<ProductBrand>>, ApiError> {
    match get_brands() {
        Ok(brands) => Ok(Json(brands)),
        Err(error) => {
            error!("{error:?}");
            Err(ApiError::InternalServer(String::from("Internal Server Error")))
        }
    }
}

#[get("/types")]
pub fn get_all_types() -> Result<Json<Vec<ProductType>>, ApiError> {
    match get_types() {
        Ok(types) => Ok(Json(types)),
        Err(error) => {
            error!("{error:?}");
            Err(ApiError::InternalServer(String::from("Internal Server Error")))
        }
    }
}