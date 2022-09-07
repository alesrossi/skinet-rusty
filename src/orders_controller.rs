use rocket::response::status;
use rocket::serde::json::{Error, Json};
use crate::db::DbError;
use crate::db::identity_functions::get_user_from_token;
use crate::db::models::DeliveryMethod;
use crate::db::order_functions::{create_order, get_delivery_methods_from_db, OrderDto, OrderToReturn};
use crate::jwt::{Response, UserToken};
use crate::responders::ApiError;
use log::{error};

#[get("/deliverymethods")]
pub fn get_delivery_methods() -> Result<Json<Vec<DeliveryMethod>>, ApiError>{
    match get_delivery_methods_from_db() {
        Ok(del_met) => Ok(Json(del_met)),
        Err(err) => {
            error!("{err:?}");
            Err(ApiError::InternalServer(String::from("Internal Server Error")))
        }
    }
}

#[post("/", data = "<order_dto>")]
pub fn post_new_order(
    order_dto: Result<Json<OrderDto>, Error>,
    token: Result<UserToken, status::Custom<Json<Response>>>,
) -> Result<Json<OrderToReturn>, ApiError> {
    match token {
        Ok(token) =>
            match get_user_from_token(token) {
                Ok(user) =>
                    match order_dto {
                        Ok(order) =>
                            match create_order(user.email, order.into_inner()) {
                                Ok(otr) => Ok(Json(otr)),
                                Err(err) => {
                                    error!("{err:?}");
                                    match err.current_context() {
                                        DbError::NotFoundError =>
                                            Err(ApiError::NotFound(String::from("User not found for this token"))),
                                        _ => Err(ApiError::InternalServer(String::from("Internal Server Error")))
                                    }
                                }
                            },
                        Err(err) => {
                            error!("{err:?}");
                            Err(ApiError::BadRequest(String::from("Invalid schema for order")))
                        }
                    },
                Err(err) => {
                    error!("{err:?}");
                    Err(ApiError::Unauthorized(String::from("Unauthorized user")))
                }
            },
        Err(err) => {
            error!("{err:?}");
            Err(ApiError::BadRequest(String::from("Invalid or missing token in header")))
        }
    }
}