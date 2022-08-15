use super::schema::products;
use serde::Serialize;
#[derive(Queryable, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: i32,
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub cost: &'a i32,
}