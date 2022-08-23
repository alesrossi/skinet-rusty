use diesel::{QueryDsl, RunQueryDsl, TextExpressionMethods, ExpressionMethods, debug_query};
use diesel::pg::Pg;
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::schema::product_brands::dsl::product_brands;
use crate::db::schema::product_types::dsl::product_types;
use crate::db::schema::products;
use crate::db::paginate::LoadPaginated;
use crate::{establish_connection, filter, sort_by};
use serde::{Serialize};
use error_stack::{IntoReport, ResultExt};
use crate::db::DbError;

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

pub fn get_product(product_id: i32) -> error_stack::Result<Product, DbError> {
    let connection = establish_connection()?;
    products::table
        .find(product_id)
        .first(&connection)
        .into_report()
        .attach_printable_lazy(|| {format!("Product '{product_id}' not found")})
        .change_context(DbError::NotFoundError)
}

pub fn get_brands() -> error_stack::Result<Vec<ProductBrand>, DbError> {
    let connection = establish_connection()?;
    product_brands
        .load::<ProductBrand>(&connection)
        .into_report()
        .attach_printable_lazy(|| {"Couldn't find brands"})
        .change_context(DbError::Other)
}

pub fn get_types() -> error_stack::Result<Vec<ProductType>, DbError> {
    let connection = establish_connection()?;
    product_types
        .load::<ProductType>(&connection)
        .into_report()
        .attach_printable_lazy(|| {"Couldn't find types"})
        .change_context(DbError::Other)

}

pub fn get_products_with_params(params: Params) -> error_stack::Result<PaginatedResult, DbError> {
    let connection = establish_connection()?;
    let mut query = products::table.into_boxed();
    // filtering
    query = filter!(query,
           (products::name, @like, params.name),
           (products::productbrand, @eq, params.brand_id),
           (products::producttype, @eq, params.type_id)
    );
    debug!("QUERY => {}", debug_query::<Pg, _>(&query));
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
        .load_with_pagination(&connection, params.page_index, params.page_size)
        .into_report()
        .attach_printable_lazy(|| {"Error during pagination"})
        .change_context(DbError::Other)?;

    Ok(PaginatedResult {
        page_index: params.page_index.unwrap_or_else(|| 1),
        page_size: result.2,
        count: result.0.len(),
        data: result.0
    })
}

