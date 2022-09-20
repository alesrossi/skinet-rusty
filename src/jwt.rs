use chrono::{Utc};
use error_stack::{IntoReport, ResultExt};
use jsonwebtoken::{Algorithm, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use rocket::{Request};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status;
use crate::db::DbError;
use jsonwebtoken::errors::Result;
use rocket::serde::json::Json;
use serde_json::Value;
use log::debug;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub subject: String,
    pub exp: i64,
    pub issuer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = status::Custom<Json<Response>>;
    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, status::Custom<Json<Response>>> {
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            debug!("Header: {authen_header:?}");
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                match decode_token(token.to_string()) {
                    Ok(token_data) => {
                        debug!("Ok");
                        return Outcome::Success(token_data.claims);
                    },
                    Err(err) => {
                        error!("{err:?}");
                    }
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(Response {
                    message: String::from("Invalid token"),
                    data: serde_json::to_value("").unwrap(),
                }),
            ),
        ))


    }
}

pub fn generate_token(email: &String) -> error_stack::Result<String, DbError> {
    let time = Utc::now().timestamp_nanos() / 1_000_000_000;
    let claims = UserToken {
        subject: email.clone(),
        exp: time + ONE_WEEK,
        issuer: env!("TOKEN_ISSUER").parse().unwrap()
    };
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret(env!("TOKEN_KEY").as_ref()))
        .into_report()
        .attach_printable_lazy(|| {format!("Error issuing token with header: {header:?}")})
        .change_context(DbError::ServerError)?;
    Ok(token)
}

fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    debug!("Decode");
    let val = Validation::new(Algorithm::HS512);
    jsonwebtoken::decode::<UserToken>
        (&token, &DecodingKey::from_secret(env!("TOKEN_KEY").as_ref()),
         &val)
}