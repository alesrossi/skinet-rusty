use serde::{Serialize};

#[derive(Queryable, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f32,
    pub description: String,
    pub picture_url: String,
    pub product_brand: i32,
    pub product_type: i32,
}

#[derive(Queryable, Serialize)]
pub struct ProductBrand {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct ProductType {
    pub id: i32,
    pub name: String,
}