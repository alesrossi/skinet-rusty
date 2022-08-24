use serde::{Serialize};
use crate::db::schema::*;

#[derive(Queryable, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub picture_url: String,
    pub product_brand: i32,
    pub product_type: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct ProductBrand {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct ProductType {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Insertable, Debug, Serialize)]
#[table_name = "app_users"]
pub struct AppUser {
    pub id: i32,
    #[column_name = "displayname"]
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub address: Option<i32>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Address {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub street: String,
    pub city: String,
    pub country: String,
    pub postal_code: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "addresses"]
pub struct AddressDto {
    #[column_name = "firstname"]
    pub first_name: String,
    #[column_name = "lastname"]
    pub last_name: String,
    pub street: String,
    pub city: String,
    pub country: String,
    #[column_name = "postalcode"]
    pub postal_code: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct DeliveryMethod {
    pub id: i32,
    pub short_name: String,
    pub delivery_time: String,
    pub description: String,
    pub price: f32
}