use rocket::response::status;
use rocket::serde::json::{Error, Json};
use crate::db::DbError;
use crate::db::identity_functions::get_user_from_token;
use crate::db::models::DeliveryMethod;
use crate::db::order_functions::{create_order, get_delivery_methods_from_db, get_post, get_posts, OrderDto, OrderToDisplay, OrderToReturn};
use crate::jwt::{Response, UserToken};
use crate::responders::ApiError;
use log::{error};

#[get("/deliveryMethods")]
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

#[get("/")]
pub fn get_orders_for_user(
    token: Result<UserToken, status::Custom<Json<Response>>>,
) -> Result<Json<Vec<OrderToDisplay>>, ApiError> {
    match token {
        Ok(token) =>
            match get_user_from_token(token) {
                Ok(user) =>
                    match get_posts(user.email) {
                        Ok(orders) => Ok(Json(orders)),
                        Err(err) => {
                            error!("{err:?}");
                            Err(ApiError::InternalServer(String::from("Internal Server Error")))
                        },
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

#[get("/<id>")]
pub fn get_order_from_id(
    id: i32,
    token: Result<UserToken, status::Custom<Json<Response>>>,
) -> Result<Json<OrderToDisplay>, ApiError> {
    match token {
        Ok(token) =>
            match get_user_from_token(token) {
                Ok(user) =>
                    match get_post(user.email, id) {
                        Ok(order) => Ok(Json(order)),
                        Err(err) => {
                            error!("{err:?}");
                            match err.current_context() {
                                DbError::NotFoundError =>
                                    Err(ApiError::NotFound(String::from("Order not found for this id"))),
                                _ => Err(ApiError::InternalServer(String::from("Internal Server Error"))),
                            }
                        },
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