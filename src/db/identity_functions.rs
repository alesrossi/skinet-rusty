use std::fmt::format;
use serde::{Serialize, Deserialize};
use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl};
use error_stack::{IntoReport, ResultExt};
use crate::db::DbError;
use crate::db::schema::app_users::dsl::app_users;
use crate::db::schema::app_users::{displayname, email, password};
use crate::establish_connection;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{DateTime, Duration, Utc};
use sha_crypt::{sha512_check, sha512_simple, Sha512Params};
use crate::db::models::AppUser;
use crate::jwt::generate_token;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDto {
    display_name: String,
    pub(crate) email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub display_name: String,
    pub email: String,
    pub token: String,
}

pub fn register_user_on_db(register_dto: RegisterDto) -> error_stack::Result<UserDto, DbError>{
    let conn = establish_connection()?;
    let params = Sha512Params::new(10_000).expect("RandomError!");
    let hashed_password = sha512_simple(register_dto.password.as_str(), &params)
        .expect("Should not fail");
    insert_into(app_users)
        .values(
            (displayname.eq(register_dto.display_name.clone()),
             email.eq(register_dto.email.clone()),
             password.eq(hashed_password))
        )
        .execute(&conn)
        .report()
        .attach_printable_lazy(|| {format!("Error inserting user {register_dto:?}")})
        .change_context(DbError::Other)?;

    Ok(UserDto {
        display_name: register_dto.display_name.clone(),
        email: register_dto.email.clone(),
        token: generate_token(&register_dto.email)?
    })
}

pub fn login_user_on_db(login_dto: LoginDto) -> error_stack::Result<UserDto, DbError> {
    let conn = establish_connection()?;
    let user: AppUser = app_users
        .filter(email.eq(login_dto.email.clone()))
        .first(&conn)
        .report()
        .attach_printable_lazy(||{format!("User '{}' was not found on db", login_dto.email)})
        .change_context(DbError::WrongLoginError)?;

    match sha512_check(&*login_dto.password, &user.password) {
        Ok(_) => Ok(UserDto {
            display_name: user.display_name,
            email: user.email.clone(),
            token: generate_token(&user.email)?
        }),
        Err(_) => Err(DbError::WrongLoginError).report()
    }
}


