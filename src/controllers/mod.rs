use actix_web::Error;
use actix_web::HttpResponse;

pub mod products_controller;

type Response = Result<HttpResponse, Error>;