use actix_web::Error;
use actix_web::HttpResponse;

pub mod products_controller;
pub mod basket_controller;
pub mod identity_controller;
pub mod orders_controller;

type Response = Result<HttpResponse, Error>;