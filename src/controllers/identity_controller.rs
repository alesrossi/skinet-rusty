use crate::db::identity_functions::*;
use rocket::serde::json::{Error, Json};
use crate::responders::ApiError;
use log::{error};
use crate::jwt::{Response, UserToken};
use rocket::response::status;
use crate::db::utils::DbError;
use crate::db::models::{AddressDto};

#[post("/register", data= "<register>")]
pub fn register_user(
    register: Result<Json<RegisterDto>, Error>,
    token: Result<UserToken, status::Custom<Json<Response>>>,
) -> Result<Json<Option<UserDto>>, ApiError> {
    match token {
        Ok(_) => Ok(Json(None)),
        Err(error) => {
            error!("{error:?}");
            match register {
                Ok(register) => {
                    match register_user_on_db(register.into_inner()) {
                        Ok(user) => Ok(Json(Some(user))),
                        Err(err) => {
                            error!("{err:?}");
                            Err(ApiError::InternalServer(String::from("Internal Server Error")))
                        }
                    }
                },
                Err(error) => {
                    error!("{error:?}");
                    Err(ApiError::BadRequest(String::from("Bad Request")))
                }
            }
        }
    }

}

#[post("/login", data= "<login>")]
pub fn login_user(
    login: Result<Json<LoginDto>, Error>,
    token: Result<UserToken, status::Custom<Json<Response>>>,
) -> Result<Json<Option<UserDto>>, ApiError> {
    match token {
        Ok(_) => Ok(Json(None)),
        Err(error) => {
            error!("{error:?}");
            match login {
                Ok(login) => {
                    match login_user_on_db(login.into_inner()) {
                        Ok(user) => Ok(Json(Some(user))),
                        Err(err) => {
                            error!("{err:?}");
                            match err.current_context() {
                                DbError::ServerError =>
                                    Err(ApiError::Unauthorized(String::from("Credentials are not correct"))),
                                _ => Err(ApiError::InternalServer(String::from("Internal Server Error")))
                            }
                        }
                    }
                },
                Err(error) => {
                    error!("{error:?}");
                    Err(ApiError::BadRequest(String::from("Bad Request")))
                }
            }
        }
    }

}

#[get("/")]
pub fn get_account(
    token: Result<UserToken, status::Custom<Json<Response>>>
) -> Result<Json<UserDto>, ApiError> {
    match token {
        Ok(token) => match get_user_from_token(token) {
            Ok(user) => Ok(Json(user)),
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
            Err(ApiError::BadRequest(String::from("Invalid or missing token in header")))
        }
    }
}

#[get("/emailexists?<email>")]
pub fn check_email_exists(
    email: Option<&str>,
) -> Result<Json<bool>, ApiError> {
    match email {
        Some(email) =>
            match check_email_existence(email) {
                Ok(_) => Ok(Json(true)),
                Err(err) =>
                    match err.current_context() {
                        DbError::NotFoundError => Ok(Json(false)),
                        _ => Err(ApiError::InternalServer(String::from("Internal Server Error")))
                    }
            },
        None => Err(ApiError::BadRequest(String::from("Email not found in query")))
    }
}

#[put("/address", data = "<address>")]
pub fn add_address(
    address: Result<Json<AddressDto>, Error>,
    token: Result<UserToken, status::Custom<Json<Response>>>
) -> Result<Json<AddressDto>, ApiError> {
    match token {
        Ok(token) =>
            match address {
                Ok(address) =>
                    match new_address_to_token(address.into_inner(), token) {
                        Ok(addr) => Ok(Json(addr)),
                        Err(err) =>
                            match err.current_context() {
                                DbError::NotFoundError =>
                                    Err(ApiError::Unauthorized(String::from("You don't have permissions for this action"))),
                                _ => Err(ApiError::InternalServer(String::from("Internal Server Error")))
                            }
                    },
                Err(err) => {
                    error!("Incorrect address schema: {err:?}");
                    Err(ApiError::BadRequest(String::from("Invalid or missing address")))
                }
            }//call address fn
        Err(err) => {
            error!("{err:?}");
            Err(ApiError::BadRequest(String::from("Invalid or missing token in header")))
        }
    }
}

#[get("/address")]
pub fn get_address(
    token: Result<UserToken, status::Custom<Json<Response>>>
) -> Result<Json<Option<AddressDto>>, ApiError> {
    match token {
        Ok(token) =>
            match get_address_from_token(token) {
                Ok(address) =>
                    match address {
                        Some(address) => Ok(Json(Some(address))),
                        None => Ok(Json(None))
                    }
                Err(err) => {
                    error!("{err:?}");
                    Err(ApiError::InternalServer(String::from("Internal Server Error")))
                }
        }
        Err(err) => {
            error!("{err:?}");
            Err(ApiError::BadRequest(String::from("Invalid or missing token in header")))
        }
    }
}