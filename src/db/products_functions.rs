use diesel::{PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods, ExpressionMethods};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::schema::product_brands::dsl::product_brands;
use crate::db::schema::product_types::dsl::product_types;
// use crate::db::schema::products::dsl::products;
use crate::db::schema::products;
use crate::db::paginate::LoadPaginated;
use crate::PaginatedResult;


#[derive(Debug)]
pub struct Params {
    pub name: Option<String>,
    pub sort_by: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
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
    if let Some(other_name) = params.name {
        query = query.filter(products::name.like(other_name));
    }

    // sorting
    if let Some(sort_by) = params.sort_by {
        println!("{}", sort_by);
        query = match sort_by.as_ref() {
            "id" => query.order(products::id.asc()),
            "id.asc" => query.order(products::id.asc()),
            "id.desc" => query.order(products::id.desc()),
            "name" => query.order(products::name.asc()),
            "name.asc" => query.order(products::name.asc()),
            "name.desc" => query.order(products::name.desc()),
            _ => query,
        };
    }

    // result
    let result = query
        .load_with_pagination(connection, params.page, params.page_size);

    match result {
        Ok(product) => {
            if product.0.is_empty() {
                Err(NotFound(String::from("Product Not Found")))
            } else {
                Ok(Json(product))
            }

        },
        Err(_) => Err(NotFound(String::from("Product Not Found"))),
    }

}