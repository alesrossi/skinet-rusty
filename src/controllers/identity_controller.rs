use actix_web::*;
use crate::controllers::Response;
use crate::db::identity_functions::{check_email_existence, get_address_from_token, get_user_from_token, login_user_on_db, LoginDto, new_address_to_token, register_user_on_db, RegisterDto};
use crate::db::models::AddressDto;
use crate::db::utils::DbError;
use crate::jwt::{UserToken};
use serde::{Deserialize};

pub fn account_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/")
                .route(web::get().to(get_account))
        )
        .service(
            web::resource("/register/")
                .route(web::post().to(register_user))
        )
        .service(
            web::resource("/login/")
                .route(web::post().to(login_user))
        )
        .service(
            web::resource("/emailexists/")
                .route(web::get().to(check_email_exists))
        )
        .service(
            web::resource("/address/")
                .route(web::put().to(add_address))
                .route(web::get().to(get_address))
        )
    ;
}

async fn register_user(token: Option<UserToken>, register_dto_from_body: web::Json<RegisterDto>) -> Response {
    match token {
        Some(_) => Ok(HttpResponse::Ok().body(())),
        None => {
            let register_dto = register_dto_from_body.into_inner();
            info!("Called register_user for user: {}", register_dto.email);
            match web::block(move || {
                register_user_on_db(register_dto)
            }).await? {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(err) => {
                    error!("{err:?}");
                    match err.current_context() {
                        DbError::EmailAlreadyInUse => Err(error::ErrorBadRequest("Email already in use")),
                        _ => Err(error::ErrorInternalServerError("Internal Server Error"))
                    }
                }
            }
        }
    }
}


async fn login_user(token: Option<UserToken>, login_dto_from_body: web::Json<LoginDto>) -> Response {
    match token {
        Some(_) => Ok(HttpResponse::Ok().body(())),
        None => {
            let login_dto = login_dto_from_body.into_inner();
            info!("Called login_user for user: {}", login_dto.email);
            match web::block(move || {
                login_user_on_db(login_dto)
            }).await? {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(err) => {
                    error!("{err:?}");
                    match err.current_context() {
                        DbError::ServerError =>
                            Err(error::ErrorUnauthorized("Credentials are not correct")),
                        _ => Err(error::ErrorInternalServerError("Internal Server Error"))
                    }
                }
            }
        }
    }
}

async fn get_account(token: UserToken) -> Response {
    info!("Called get_account with token {:?}", token);
    match web::block(move || {
        get_user_from_token(token)
    }).await? {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            error!("{err:?}");
            match err.current_context() {
                DbError::NotFoundError =>
                    Err(error::ErrorNotFound("User not found for this token")),
                _ => Err(error::ErrorInternalServerError("Internal Server Error"))
            }
        }
    }
}

// maybe add catcher

#[derive(Debug, Deserialize)]
struct EmailQuery{
    email: String
}

async fn check_email_exists(email_from_query: web::Query<EmailQuery>) -> Response {
    let email = email_from_query.into_inner().email;
    info!("Called check_email_exists with email '{}'", email);
    match web::block(move || {
        check_email_existence(email.as_str())
    }).await? {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}

async fn add_address(token: UserToken, address_dto_from_body: web::Json<AddressDto>) -> Response {
    let address = address_dto_from_body.into_inner();
    info!("Called add_address with address: {:?}", address);
    match web::block(move || {
        new_address_to_token(address, token)
    }).await? {
        Ok(addr) => Ok(HttpResponse::Ok().json(addr)),
        Err(err) =>
            match err.current_context() {
                DbError::NotFoundError =>
                    Err(error::ErrorInternalServerError("You don't have permission for this action")),
                _ => Err(error::ErrorInternalServerError("Internal Server Error"))
            }
    }
}

async fn get_address(token: UserToken) -> Response {
    info!("Called get_address for token: {:?}", token);
    match web::block(move || {
        get_address_from_token(token)
    }).await? {
        Ok(addr) => Ok(HttpResponse::Ok().json(addr)),
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}
