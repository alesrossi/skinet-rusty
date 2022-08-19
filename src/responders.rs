#[allow(unused)]
use rocket::http::ContentType;

#[derive(Responder)]
pub enum ApiError<'r> {
    #[response(status = 400)]
    BadRequest(&'r str),
    #[response(status = 401)]
    Unauthorized(&'r str),
    #[response(status = 404)]
    NotFound(&'r str),
    #[response(status = 500)]
    InternalServer(&'r str),
}

