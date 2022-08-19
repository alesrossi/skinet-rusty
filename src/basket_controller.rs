use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use crate::db::redis::*;

#[get("/?<id>")]
pub fn get_basket_from_id(id: Option<String>) -> Result<Json<CustomerBasket>, BadRequest<String>> {
    let conn = connect_redis();
    match id {
        Some(my_id)=> Ok(Json(get_basket(my_id, conn))),
        None => Err(BadRequest(Some(String::from("Id not found in query"))))
    }
}

#[post("/", data = "<basket>")]
pub fn return_basket(basket: Json<CustomerBasket>) -> Json<CustomerBasket> {
    let conn = connect_redis();
    Json(create_basket(basket.into_inner(), conn))
}