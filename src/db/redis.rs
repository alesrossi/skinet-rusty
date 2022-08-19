use std::collections::LinkedList;
use serde::{Serialize, Deserialize};
use simple_redis::client::Client;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasketItem {
    pub id: u64,
    pub product_name: String,
    pub cost: f32,
    pub quantity: i32,
    pub picture_url: String,
    pub product_brand: String,
    pub product_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerBasket {
    pub id: String,
    pub items: LinkedList<BasketItem>
}

impl From<String> for CustomerBasket {
    fn from(id: String) -> Self {
        CustomerBasket {
            id,
            items: LinkedList::new()
        }
    }
}

pub fn connect_redis() -> Client {
    simple_redis::create(env!("REDIS_URL")).expect("Unexpected error connecting to redis")
}

pub fn get_basket(key: String, mut con: Client) -> CustomerBasket {
    let res = con.get::<String>(&*key).unwrap();
    serde_json::from_str(&*res).unwrap()
}

pub fn create_basket(basket: CustomerBasket, mut con: Client) -> CustomerBasket {
    con.set(
        &*basket.id,
        serde_json::to_string(&basket)
            .expect("error parsing")
            .as_str()
    ).expect("panic message");
    basket
}