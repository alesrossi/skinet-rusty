use diesel::RunQueryDsl;
use error_stack::{IntoReport, ResultExt};
use crate::db::DbError;
use crate::db::models::DeliveryMethod;
use crate::db::schema::delivery_methods::dsl::delivery_methods;
use crate::establish_connection;

pub fn get_delivery_methods_from_db() -> error_stack::Result<Vec<DeliveryMethod>, DbError> {
    let conn = establish_connection()?;
    delivery_methods.
        load::<DeliveryMethod>(&conn)
        .into_report()
        .attach_printable_lazy(|| {"Couldn't find delivery methods"})
        .change_context(DbError::NotFoundError)
}