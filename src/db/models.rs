use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f32,
    pub description: String,
    pub picture_url: String,
    pub product_brand: i32,
    pub product_type: i32,
}

// #[derive(Insertable)]
// #[table_name="products"]
// pub struct NewProduct<'a> {
//     pub name: &'a str,
//     pub cost: &'a f32,
// }

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