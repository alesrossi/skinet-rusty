#[allow(unused)]
use rocket::http::ContentType;

#[derive(Responder, Debug)]
pub enum ApiError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    InternalServer(String),
}

