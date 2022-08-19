use std::{collections::LinkedList, fmt, error::Error};
use std::fmt::{Debug, Formatter};
use error_stack::{IntoReport, ResultExt};
use serde::{Serialize, Deserialize};
use simple_redis::client::Client;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasketItem {
    pub id: u64,
    pub product_name:  String,
    pub cost: f32,
    pub quantity: i32,
    pub picture_url: String,
    pub product_brand: String,
    pub product_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerBasket {
    pub id:  String,
    pub items: LinkedList<BasketItem>
}


pub fn connect_redis() -> error_stack::Result<Client, RedisError> {
    simple_redis::create(env!("REDIS_URL"))
        .report()
        .change_context(RedisError)
}

pub fn get_basket(key: &str) -> error_stack::Result<CustomerBasket, RedisError> {

    match connect_redis()?.get_string(key) {
        Ok(res) => {
            let r = serde_json::from_str::<CustomerBasket>(&*res);
            Ok(r.unwrap())
        },
        _ => {
            let basket = CustomerBasket {
                id: key.parse().unwrap(),
                items: Default::default()
            };
            create_basket(basket)
        }
    }
}

pub fn create_basket(basket: CustomerBasket) -> error_stack::Result<CustomerBasket, RedisError> {
    connect_redis()?
        .set(&*basket.id,
             serde_json::to_string(&basket).unwrap().as_str())
        .report().change_context(RedisError)?;
    Ok(basket)

}

pub fn delete_basket(key: &str) -> error_stack::Result<(), RedisError> {
    connect_redis()?
        .del(key)
        .report().change_context(RedisError)
}

#[derive(Debug)]
pub struct RedisError;

impl fmt::Display for RedisError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("Redis Error")
    }
}

impl Error for RedisError {}