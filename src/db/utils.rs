
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use std::{fmt, fmt::Formatter, error::Error};
use error_stack::{IntoReport, ResultExt};


pub fn establish_connection() -> error_stack::Result<PgConnection, DbError> {
    let database_url = env!("DATABASE_URL").to_string();
    PgConnection::establish(&database_url)
        .into_report()
        .attach_printable_lazy(|| {format!("Error connecting to database: {database_url}")})
        .change_context(DbError::ServerError)
}

#[derive(Debug)]
pub enum DbError {
    NotFoundError,
    ServerError
}

impl fmt::Display for DbError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result { fmt.write_str("Db Error") }
}

impl Error for DbError {}