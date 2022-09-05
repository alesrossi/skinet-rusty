use diesel::RunQueryDsl;
use error_stack::{IntoReport, ResultExt};
use crate::db::DbError;
use crate::db::models::{AddressDto, DeliveryMethod};
use crate::db::schema::delivery_methods::dsl::delivery_methods;
use crate::establish_connection;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct OrderDto {
    basket_id: String,
    delivery_methods_id: i32,
    ship_to_address: AddressDto,
}

pub fn get_delivery_methods_from_db() -> error_stack::Result<Vec<DeliveryMethod>, DbError> {
    let conn = establish_connection()?;
    delivery_methods
        .load::<DeliveryMethod>(&conn)
        .into_report()
        .attach_printable_lazy(|| {"Couldn't find delivery methods"})
        .change_context(DbError::NotFoundError)
}

// pub fn create_order() -> error_stack::Result<Order>