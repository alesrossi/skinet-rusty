use diesel::{PgConnection, RunQueryDsl};
use crate::db::models::Product;
use crate::db::schema::products::dsl::products;

pub fn get_products(connection: &PgConnection) -> Vec<Product> {
    products
        .load::<Product>(connection)
        .expect("Error loading posts")


}
