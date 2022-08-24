use rocket::serde::json::Json;
use crate::db::models::DeliveryMethod;
use crate::db::order_functions::get_delivery_methods_from_db;
use crate::responders::ApiError;

#[get("/deliverymethods")]
pub fn get_delivery_methods() -> Result<Json<Vec<DeliveryMethod>>, ApiError>{
    match get_delivery_methods_from_db() {
        Ok(del_met) => Ok(Json(del_met)),
        Err(err) => {
            error!("{err:?}");
            Err(ApiError::InternalServer(String::from("Internal Server Error")))
        }
    }
}