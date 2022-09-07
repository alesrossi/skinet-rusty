use std::collections::linked_list::LinkedList;
use diesel::{dsl, insert_into, QueryDsl, RunQueryDsl, ExpressionMethods};
use error_stack::{IntoReport, Report, ResultExt};
use crate::db::DbError;
use crate::db::models::{Address, AddressDto, DeliveryMethod, Order, OrderItem, Product, ProductOrderItem};
use crate::db::schema::delivery_methods::dsl::delivery_methods;
use crate::establish_connection;
use serde::{Serialize, Deserialize};
use crate::db::redis::get_basket;
use crate::db::schema::addresses::dsl::addresses;
use crate::db::schema::order_items::dsl::order_items;
use crate::db::schema::order_items::{parentorder, price, productitemordered, quantity};
use crate::db::schema::orders::dsl::orders;
use crate::db::schema::orders::*;
use crate::db::schema::product_order_items::dsl::product_order_items;
use crate::db::schema::product_order_items::{pictureurl, productid, productname};
use crate::db::schema::products::dsl::products;

const VALID_DELIVERY: i32 = 4;

#[derive(Debug, Serialize)]
pub enum OrderStatus {
    Pending,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDto {
    pub basket_id: String,
    pub delivery_method_id: i32,
    pub ship_to_address: AddressDto,
}

#[derive(Debug, Serialize)]
pub struct OrderToReturn {
    pub id: i32,
    pub buyer_email: String,
    pub ship_to_address: AddressDto,
    pub delivery_method: DeliveryMethod,
    pub order_items: LinkedList<OrderItem>,
    pub subtotal: f32,
    pub status: OrderStatus,
    pub payment_intent_id: String,
}

pub fn get_delivery_methods_from_db() -> error_stack::Result<Vec<DeliveryMethod>, DbError> {
    let conn = establish_connection()?;
    delivery_methods
        .load::<DeliveryMethod>(&conn)
        .into_report()
        .attach_printable_lazy(|| {"Couldn't find delivery methods"})
        .change_context(DbError::NotFoundError)
}

pub fn create_order(email: String, order: OrderDto) -> error_stack::Result<OrderToReturn, DbError> {
    let conn = establish_connection()?;
    let basket = get_basket(order.basket_id.as_str())
        .change_context(DbError::ServerError)?;

    if order.delivery_method_id < 1 || order.delivery_method_id >  VALID_DELIVERY
    { return Err(Report::from(DbError::ServerError)) }

    let del: DeliveryMethod = delivery_methods.find(order.delivery_method_id)
        .first(&conn)
        .into_report()
        .change_context(DbError::NotFoundError)?;

    let mut sub_total = 0.;
    basket.items.iter().for_each(|e| sub_total += e.price * e.quantity as f32);

    let addr: Address = insert_into(addresses)
        .values(&order.ship_to_address)
        .get_result(&conn)
        .into_report()
        .attach_printable_lazy(|| {format!("Error inserting address: {:?}", &order.ship_to_address)})
        .change_context(DbError::ServerError)?;

    let inserted_order: Order = insert_into(orders)
        .values(
            (orderdate.eq(dsl::now),
             address.eq(addr.id),
             deliverymethod.eq(del.id),
             subtotal.eq(sub_total),
            total.eq(sub_total+del.price),
            paymentintentid.eq(""))
        )
        .get_result(&conn)
        .into_report()
        .attach_printable_lazy(|| {format!("Error inserting order {order:?}")})
        .change_context(DbError::ServerError)?;

    let mut other_order_items: LinkedList<OrderItem> = LinkedList::new();
    for item in basket.items {
        let product_item: Product = products.find(item.id)
            .first(&conn)
            .into_report()
            .change_context(DbError::NotFoundError)?;

        let poi: ProductOrderItem = insert_into(product_order_items)
            .values((
                productid.eq(product_item.id),
                productname.eq(product_item.name),
                pictureurl.eq(product_item.picture_url)
                ))
            .get_result(&conn)
            .into_report()
            .change_context(DbError::ServerError)?;

        let res: OrderItem = insert_into(order_items)
            .values((
                productitemordered.eq(poi.id),
                price.eq(product_item.price),
                quantity.eq(item.quantity),
                parentorder.eq(inserted_order.id)
            ))
            .get_result(&conn)
            .into_report()
            .change_context(DbError::ServerError)?;
        other_order_items.push_back(res);
    }

    Ok(OrderToReturn{
        id: inserted_order.id,
        buyer_email: email,
        ship_to_address: AddressDto::from(addr),
        delivery_method: del,
        order_items: other_order_items,
        subtotal: sub_total,
        status: OrderStatus::Pending,
        payment_intent_id: "".to_string()
    })
}