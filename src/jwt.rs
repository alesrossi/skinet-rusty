use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use chrono::Utc;
use error_stack::{IntoReport, ResultExt};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{Algorithm, DecodingKey, encode, EncodingKey, Header, TokenData, Validation, errors::Result as TokenResult};
use crate::db::utils::DbError;
use serde::{Serialize, Deserialize};
use serde_json::Value;

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

impl FromRequest for UserToken {
    type Error = Error;
    type Future = Ready<Result<UserToken, Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                match decode_token(token.to_string()) {
                    Ok(token) => {
                        debug!("Successfully decoded token: {:?}", token);
                        ok(token.claims)
                    },
                    Err(_e) => err(ErrorUnauthorized("Invalid or expired token!")),
                }
            }
            None => err(ErrorUnauthorized("Missing token!")),
        }
    }
}

pub fn generate_token(email: &str) -> error_stack::Result<String, DbError> {
    let time = Utc::now().timestamp_nanos() / 1_000_000_000;
    let claims = UserToken {
        subject: email.to_string(),
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

fn decode_token(token: String) -> TokenResult<TokenData<UserToken>> {
    debug!("Decode");
    let val = Validation::new(Algorithm::HS512);
    jsonwebtoken::decode::<UserToken>
        (&token, &DecodingKey::from_secret(env!("TOKEN_KEY").as_ref()),
         &val)
}

#[cfg(test)]
mod tests {
    use actix_web::{FromRequest};
    use actix_web::http::header::ContentType;
    use actix_web::test::TestRequest;
    use crate::jwt::{decode_token, generate_token, UserToken};

    #[actix_web::test]
    async fn from_responder_ok() {
        let token = "Bearer ".to_owned() + generate_token("test@email.com").unwrap().as_str();
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .insert_header(("Authorization", token));


        let mut req_parts = req.to_http_parts();
        let res = UserToken::from_request(&req_parts.0, &mut req_parts.1).await;
        assert_eq!("test@email.com", res.unwrap().subject);
    }

    #[test]
    fn decode_token_invalid() {
        let res = decode_token(String::from("Invalid token"));
        assert!(res.is_err());
    }

    #[test]
    fn decode_token_ok() {
        let token = generate_token("test@email.com").unwrap();
        let res = decode_token(token);
        let token_valid = UserToken{
            subject: "test@email.com".to_string(),
            exp: 0,
            issuer: "".to_string()
        };
        assert_eq!(res.unwrap().claims.subject, token_valid.subject);
    }

    #[test]
    fn generate_token_ok() {
        assert!(generate_token("test@email.com").is_ok());
    }

}