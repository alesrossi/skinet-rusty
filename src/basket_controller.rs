use rocket::serde::json::{Error, Json};
use crate::db::redis::*;
use crate::responders::ApiError;
use log::{error};

#[get("/?<id>")]
pub fn get_basket_from_id(id: Option<&str>) -> Result<Json<CustomerBasket>, ApiError> {
    match id {
        Some(my_id)=> match get_basket(my_id) {
            Ok(basket) => Ok(Json(basket)),
            Err(err) => {
                println!("{err:?}");
                Err(ApiError::InternalServer("Internal Server Error"))
            }
        },
        None => Err(ApiError::BadRequest("id not found in query"))
    }
}

#[post("/", data = "<basket>")]
pub fn create_new_basket(basket: Result<Json<CustomerBasket>, Error>) -> Result<Json<CustomerBasket>, ApiError<'static>> {
    match basket {
        Ok(basket) =>  match create_basket(basket.into_inner()) {
            Ok(basket) => {
                Ok(Json(basket))
            },
            Err(err) => {
                error!("{err:?}");
                Err(ApiError::InternalServer("Internal Server Error"))
            }
        },
        Err(error) => {
            error!("{error:?}");
            Err(ApiError::BadRequest("Invalid basket schema"))
        }
    }
}

#[delete("/?<id>")]
pub fn delete_basket_from_id(id: Option<&str>) -> Result<(), ApiError> {
    match id {
        Some(my_id)=> match delete_basket(my_id) {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("{err:?}");
                Err(ApiError::InternalServer("Internal Server Error"))
            }
        },
        None => Err(ApiError::BadRequest("id not found in query"))
    }
}