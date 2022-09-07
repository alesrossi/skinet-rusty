use serde::{Serialize, Deserialize};
use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl};
use error_stack::{IntoReport, ResultExt};
use crate::db::DbError;
use crate::db::schema::app_users::dsl::app_users;
use crate::db::schema::app_users::{address, displayname, email, password};
use crate::establish_connection;
use sha_crypt::{sha512_check, sha512_simple, Sha512Params};
use crate::db::models::{Address, AddressDto, AppUser};
use crate::db::schema::addresses::dsl::addresses;
use crate::db::schema::addresses::id;
use crate::jwt::{generate_token, UserToken};


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
        .into_report()
        .attach_printable_lazy(|| {format!("Error inserting user {register_dto:?}")})
        .change_context(DbError::ServerError)?;

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
        .into_report()
        .attach_printable_lazy(||{format!("User '{}' was not found on db", login_dto.email)})
        .change_context(DbError::ServerError)?;

    match sha512_check(&login_dto.password, &user.password) {
        Ok(_) => Ok(UserDto {
            display_name: user.display_name,
            email: user.email.clone(),
            token: generate_token(&user.email)?
        }),
        Err(_) => Err(DbError::ServerError).into_report()
    }
}

pub fn get_user_from_token(token: UserToken) -> error_stack::Result<UserDto, DbError> {
    let conn = establish_connection()?;
    let user: AppUser = app_users
        .filter(email.eq(token.subject))
        .first(&conn)
        .into_report()
        .attach_printable_lazy(||{"User related to token was not found on db"})
        .change_context(DbError::NotFoundError)?;
    Ok(UserDto {
        display_name: user.display_name,
        email: user.email.clone(),
        token: generate_token(&user.email)?
    })
}

pub fn check_email_existence(user_email: &str) -> error_stack::Result<(), DbError> {
    let conn = establish_connection()?;
    let user: error_stack::Result<AppUser, DbError> = app_users
        .filter(email.eq(user_email))
        .first(&conn)
        .into_report()
        .attach_printable_lazy(|| {"User not found for this email"})
        .change_context(DbError::NotFoundError);

    match user {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

pub fn new_address_to_token(
    user_address: AddressDto, token: UserToken
) -> error_stack::Result<AddressDto, DbError> {
    let conn = establish_connection()?;
    let mut user: AppUser = app_users
        .filter(email.eq(token.subject))
        .first(&conn)
        .into_report()
        .attach_printable_lazy(|| {"User not found for this token"})
        .change_context(DbError::NotFoundError)?;

    let addr: Address = insert_into(addresses)
        .values(&user_address)
        .get_result(&conn)
        .into_report()
        .attach_printable_lazy(|| {format!("Error inserting address: {:?}", &address)})
        .change_context(DbError::ServerError)?;

    user.address = Some(addr.id);

    diesel::update(&user)
        .set(address.eq(addr.id))
        .execute(&conn)
        .into_report()
        .attach_printable_lazy(|| {"Error updating user"})
        .change_context(DbError::ServerError)?;
    Ok(user_address)
}

pub fn get_address_from_token(
    token: UserToken
) -> error_stack::Result<Option<AddressDto>, DbError> {
    let conn = establish_connection()?;
    let user: AppUser = app_users
        .filter(email.eq(token.subject))
        .first(&conn)
        .into_report()
        .attach_printable_lazy(|| {"User not found for this token"})
        .change_context(DbError::NotFoundError)?;

    if user.address.is_none() { return Ok(None) }

    let addr: Address = addresses
        .filter(id.eq(user.address.unwrap()))
        .first(&conn)
        .into_report()
        .attach_printable_lazy(|| {"Address not found in this db"})
        .change_context(DbError::NotFoundError)?;

    Ok(Some(AddressDto {
        first_name: addr.first_name,
        last_name: addr.last_name,
        street: addr.street,
        city: addr.city,
        country: addr.country,
        postal_code: addr.postal_code
    }))
}