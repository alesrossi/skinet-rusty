use actix_web::*;
use crate::controllers::Response;
use crate::db::redis::{create_basket, CustomerBasket, delete_basket, get_basket};
use serde::{Deserialize};

pub fn basket_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/")
                .route(web::get().to(get_basket_from_id))
                .route(web::post().to(create_new_basket))
                .route(web::delete().to(delete_basket_from_id))
        )
    ;
}

#[derive(Debug, Deserialize)]
struct BasketQuery{
    id: String
}

async fn get_basket_from_id(query: web::Query<BasketQuery>) -> Response {
    let basket = query.into_inner();
    info!("Called get_basket_from_id with id, {:?}", basket.id);
    match web::block(move || {
        get_basket(basket.id.as_str())
    }).await? {
        Ok(basket) => Ok(HttpResponse::Ok().json(basket)),
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}

async fn create_new_basket(basket_from_body: web::Json<CustomerBasket>) -> Response {
    let basket = basket_from_body.into_inner();
    info!("Called create_new_basket with basket, {:?}", basket);
    match web::block(move || {
        create_basket(basket)
    }).await? {
        Ok(basket) => Ok(HttpResponse::Ok().json(basket)),
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}

async fn delete_basket_from_id(query: web::Query<BasketQuery>) -> Response {
    let basket = query.into_inner();
    info!("Called delete_basket_from_id with id, {:?}", basket.id);
    match web::block(move || {
        delete_basket(basket.id.as_str())
    }).await? {
        Ok(basket) => Ok(HttpResponse::Ok().json(basket)),
        Err(err) => {
            error!("{err:?}");
            Err(error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}