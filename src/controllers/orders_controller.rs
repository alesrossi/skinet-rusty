use actix_web::*;
use crate::controllers::Response;
use crate::db::identity_functions::get_user_from_token;
use crate::db::order_functions::{create_order, get_delivery_methods_from_db, get_post, get_posts, OrderDto};
use crate::db::utils::DbError;
use crate::jwt::UserToken;

pub fn orders_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/")
                .route(web::get().to(get_orders_for_user))
                .route(web::post().to(post_new_order))
        )
        .service(
            web::resource("/deliverymethods/")
                .route(web::get().to(get_delivery_methods))
        )
        .service(
            web::resource("/deliveryMethods/")
                .route(web::get().to(get_delivery_methods))
        )
        .service(
            web::resource("/{id}/")
                .route(web::get().to(get_order_from_id))
        )
    ;
}

async fn get_delivery_methods() -> Response {
    info!("Called get_delivery_methods");
    match web::block(move || {
        get_delivery_methods_from_db()
    }).await? {
        Ok(del_met) => Ok(HttpResponse::Ok().json(del_met)),
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}

async fn post_new_order(token: UserToken, order_dto_from_body: web::Json<OrderDto>) -> Response {
    let order_dto = order_dto_from_body.into_inner();
    info!("Called post_new_order with order_dto: {:?}", order_dto);
    match get_user_from_token(token) {
        Ok(user) =>
            match create_order(user.email, order_dto) {
                Ok(otr) => Ok(HttpResponse::Ok().json(otr)),
                Err(err) => {
                    error!("{err:?}");
                    match err.current_context() {
                        DbError::NotFoundError =>
                            Err(error::ErrorNotFound("User Not Found for this token")),
                        _ => Err(error::ErrorInternalServerError("Internal Server Error"))
                    }
                }
        },
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorUnauthorized("Unauthorised user"))
        }
    }
}

async fn get_orders_for_user(token: UserToken) -> Response {
    info!("Called get_orders_for_user with token: {:?}", token);
    match get_user_from_token(token) {
        Ok(user) =>
            match get_posts(user.email) {
                Ok(orders) => Ok(HttpResponse::Ok().json(orders)),
                Err(err) => {
                    error!("{err:?}");
                    Err(error::ErrorInternalServerError("Internal Server Error"))
                },
            },
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorUnauthorized("Unauthorised user"))
        }
    }
}

async fn get_order_from_id(token: UserToken, id_from_path: web::Path<i32>) -> Response {
    let id = id_from_path.into_inner();
    info!("Called get_order_from_id with token: {:?} and id: '{}'", token, id);
    match get_user_from_token(token) {
        Ok(user) =>
            match get_post(user.email, id) {
                Ok(order) => Ok(HttpResponse::Ok().json(order)),
                Err(err) => {
                    error!("{err:?}");
                    match err.current_context() {
                        DbError::NotFoundError =>
                            Err(error::ErrorNotFound("Order Not Found for this id")),
                        _ => Err(error::ErrorInternalServerError("Internal Server Error"))
                    }
                },
            },
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorUnauthorized("Unauthorised user"))
        }
    }
}