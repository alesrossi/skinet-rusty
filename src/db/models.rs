use serde::{Serialize};
use crate::db::schema::*;
use chrono::NaiveDateTime;

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

#[derive(Queryable, Insertable, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "orders"]
pub struct Order {
    pub id: i32,
    #[column_name = "orderdate"]
    pub order_date: NaiveDateTime,
    pub address: i32,
    #[column_name = "deliverymethod"]
    pub delivery_method: i32,
    pub subtotal: f32,
    pub total: f32,
    #[column_name = "paymentintentid"]
    pub payment_intent_id: String
}

#[derive(Queryable, Insertable, Debug, Serialize)]
pub struct ProductOrderItem {
    pub id: i32,
    #[column_name = "productid"]
    pub product_id: i32,
    #[column_name = "productname"]
    pub product_name: String,
    #[column_name = "pictureurl"]
    pub picture_url: String
}

#[derive(Queryable, Debug, Serialize)]
pub struct OrderItem {
    pub id: i32,
    pub product_item_ordered: i32,
    pub price: f32,
    pub quantity: i32,
    pub parent_order: i32
}

impl From<Address> for AddressDto {
    fn from(addr: Address) -> Self {
        AddressDto {
            first_name: addr.first_name,
            last_name: addr.last_name,
            street: addr.street,
            city: addr.city,
            country: addr.country,
            postal_code: addr.postal_code
        }
    }
}