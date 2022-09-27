//use actix_web::{get, post, web, HttpResponse, HttpServer, Responder, web::service};
use actix_web::*;
use crate::db::utils::{DbError};
use crate::controllers::Response;
use crate::db::products_functions::{get_brands, get_product, get_products_with_params, get_types, Params};

pub fn products_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
        web::resource("/")
            .route(web::get().to(get_all_products))
        )
        .service(
            web::resource("/brands/")
                .route(web::get().to(get_all_brands))
        )
        .service(
            web::resource("/types/")
                .route(web::get().to(get_all_types))
        )
        .service(
            web::resource("/{product_id}/")
                .route(web::get().to(get_one))
        )
    ;
}

async fn get_all_products(params_from_path: web::Query<Params>) -> Response {
    let params = params_from_path.into_inner();
    info!("Called get_all_products with params {:?}", params);
    match web::block(move || {
        get_products_with_params(params)
    }).await? {
        Ok(products) => Ok(HttpResponse::Ok().json(products)),
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        },
    }

}

async fn get_one(product_id: web::Path<i32>) -> Response {
    let prod_id = product_id.into_inner();
    info!("Called get_product with id, {:?}", prod_id);
    match web::block(move || {
        get_product(prod_id)
    }).await? {
        Ok(product) => Ok(HttpResponse::Ok().json(product)),
        Err(err) => {
          error!("{err:?}");
            match err.current_context() {
                DbError::NotFoundError => Err(error::ErrorNotFound("Product not found")),
                _ => Err(error::ErrorInternalServerError("Internal Server Error")),
            }
        },
    }
}

async fn get_all_brands() -> Response {
    info!("Called get_all_brands");
    match web::block(move || {
        get_brands()
    }).await? {
        Ok(brands) => Ok(HttpResponse::Ok().json(brands)),
        Err(error) => {
            error!("{error:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}

async fn get_all_types() -> Response {
    info!("Called get_all_types");
    match web::block(move || {
        get_types()
    }).await? {
        Ok(types) => Ok(HttpResponse::Ok().json(types)),
        Err(error) => {
            error!("{error:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}