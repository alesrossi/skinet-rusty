use diesel::{QueryDsl, RunQueryDsl, PgTextExpressionMethods, ExpressionMethods, debug_query};
use diesel::pg::Pg;
use error_stack::{IntoReport, ResultExt};
use crate::db::models::{Product, ProductBrand, ProductType};
use crate::db::schema::product_brands::dsl::product_brands;
use crate::db::schema::product_types::dsl::product_types;
use crate::db::schema::products;
use crate::db::paginate::*;
use serde::{Serialize, Deserialize};
use crate::db::utils::{DbError, establish_connection};
use crate::{filter, sort_by};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    let mut connection = establish_connection() ?;
    debug!("Returning product");
    products::table
        .find(product_id)
        .first(&mut connection)
        .into_report()
        .attach_printable_lazy(|| {format!("Product '{product_id}' not found")})
        .change_context(DbError::NotFoundError)
}

pub fn get_brands() -> error_stack::Result<Vec<ProductBrand>, DbError> {
    let mut connection = establish_connection()?;
    debug!("Returning brands");
    product_brands
        .load::<ProductBrand>(&mut connection)
        .into_report()
        .attach_printable_lazy(|| {"Error fetching brands"})
        .change_context(DbError::ServerError)
}

pub fn get_types() -> error_stack::Result<Vec<ProductType>, DbError> {
    let mut connection = establish_connection()?;
    debug!("Returning types");
    product_types
        .load::<ProductType>(&mut connection)
        .into_report()
        .attach_printable_lazy(|| {"Error fetching types"})
        .change_context(DbError::ServerError)

}

pub fn get_products_with_params(params: Params) -> error_stack::Result<PaginatedResult, DbError> {
    let mut connection = establish_connection()?;
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
    // pagination
    let mut res = query.paginate(params.page_index.unwrap_or(1));
    match params.page_size {
        Some(per_page) => res = res.per_page(per_page),
        None => res = res.per_page(DEFAULT_PER_PAGE),
    }

    let (result, _total_pages) = res.load_and_count_pages::<Product>(&mut connection)
        .into_report()
        .attach_printable_lazy(|| {"Error during pagination"})
        .change_context(DbError::ServerError)?;

    Ok(PaginatedResult {
        page_index: params.page_index.unwrap_or(1),
        page_size: params.page_size.unwrap_or(DEFAULT_PER_PAGE),
        count: result.len(),
        data: result
    })
}

