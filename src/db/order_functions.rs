use std::collections::linked_list::LinkedList;
use chrono::NaiveDateTime;
use diesel::{dsl, insert_into, QueryDsl, RunQueryDsl, ExpressionMethods, JoinOnDsl, PgConnection};
use error_stack::{IntoReport, Report, ResultExt};
use crate::db::utils::{DbError, establish_connection};
use crate::db::models::{Address, AddressDto, DeliveryMethod, Order, OrderItem, OrderStatus, Product, ProductOrderItem};
use crate::db::schema::delivery_methods::dsl::delivery_methods;
use crate::db::schema::delivery_methods as dm;
use serde::{Serialize, Deserialize};
use crate::db::redis::get_basket;
use crate::db::schema::addresses;
use crate::db::schema::order_items::dsl::order_items;
use crate::db::schema::order_items::{parentorder, price, productitemordered, quantity};
use crate::db::schema::orders::dsl::orders;
use crate::db::schema::orders::*;
use crate::db::schema::product_order_items::dsl::product_order_items;
use crate::db::schema::product_order_items::{pictureurl, productid, productname};
use crate::db::schema::products::dsl::products;
use diesel::debug_query;
use diesel::pg::Pg;


const VALID_DELIVERY: i32 = 4;

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderToDisplay {
    pub id: i32,
    pub buyer_email: String,
    pub order_date: NaiveDateTime,
    pub ship_to_address: AddressDto,
    pub delivery_method: String,
    pub shipping_price: f32,
    pub order_items: Vec<OrderItem>,
    pub subtotal: f32,
    pub status: OrderStatus,
    pub total: f32,
}

pub fn get_delivery_methods_from_db() -> error_stack::Result<Vec<DeliveryMethod>, DbError> {
    let conn = establish_connection()?;
    debug!("Getting delivery methods from DB");
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
    debug!("Retrieved basket from id: {:?}", basket);
    if order.delivery_method_id < 1 || order.delivery_method_id >  VALID_DELIVERY
    { return Err(Report::from(DbError::ServerError)) }

    let del: DeliveryMethod = delivery_methods.find(order.delivery_method_id)
        .first(&conn)
        .into_report()
        .change_context(DbError::NotFoundError)?;
    debug!("Retrieved delivery method from request: {:?}", del);

    let mut sub_total = 0.;
    basket.items.iter().for_each(|e| sub_total += e.price * e.quantity as f32);

    let addr: Address = insert_into(addresses::table)
        .values(&order.ship_to_address)
        .get_result(&conn)
        .into_report()
        .change_context(DbError::ServerError)?;
    debug!("Inserted address from request: {:?}", addr);

    let inserted_order: Order = insert_into(orders)
        .values(
            (buyeremail.eq(email.clone()),
                orderdate.eq(dsl::now),
             address.eq(addr.id),
             deliverymethod.eq(del.id),
             subtotal.eq(sub_total),
            total.eq(sub_total+del.price),
                status.eq(OrderStatus::Pending),
            paymentintentid.eq(""))
        )
        .get_result(&conn)
        .into_report()
        .change_context(DbError::ServerError)?;
    debug!("Inserted order from request: {:?}", inserted_order);

    let mut other_order_items: LinkedList<OrderItem> = LinkedList::new();
    debug!("Inserting order items : {:?}", basket.items);
    for item in basket.items {
        let product_item: Product = products.find(item.id)
            .first(&conn)
            .into_report()
            .change_context(DbError::NotFoundError)?;
        debug!("Getting product from item: {:?}", product_item);

        let poi: ProductOrderItem = insert_into(product_order_items)
            .values((
                productid.eq(product_item.id),
                productname.eq(product_item.name),
                pictureurl.eq(product_item.picture_url)
                ))
            .get_result(&conn)
            .into_report()
            .change_context(DbError::ServerError)?;
        debug!("Inserted poi from item: {:?}", poi);

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
        debug!("Inserted order item from item: {:?}", res);

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

pub fn get_posts(email: String) -> error_stack::Result<Vec<OrderToDisplay>, DbError> {
    let connection = establish_connection()?;
    let query = orders
        .filter(buyeremail.eq(email))
        .inner_join(delivery_methods.on(dm::id.eq(deliverymethod)))
        .inner_join(addresses::table.on(addresses::id.eq(address)));
    debug!("get_posts query: {}", debug_query::<Pg, _>(&query).to_string());
    let ords: Vec<(Order, DeliveryMethod, Address)> =
        query
        .load::<(Order, DeliveryMethod, Address)>(&connection)
        .into_report()
        .change_context(DbError::NotFoundError)?;
    debug!("Getting orders for user: {:?}", ords);
    display_orders(ords, &connection)
}

pub fn get_post(email: String, order_id: i32) -> error_stack::Result<OrderToDisplay, DbError> {
    let connection = establish_connection()?;
    let query = orders
        .filter(buyeremail.eq(email))
        .filter(id.eq(order_id.clone()))
        .inner_join(delivery_methods.on(dm::id.eq(deliverymethod)))
        .inner_join(addresses::table.on(addresses::id.eq(address)));
    debug!("get_post query:{}", debug_query::<Pg, _>(&query).to_string());
    let ord: (Order, DeliveryMethod, Address) =
        query
            .first::<(Order, DeliveryMethod, Address)>(&connection)
            .into_report()
            .change_context(DbError::NotFoundError)?;
    debug!("Getting order for user: {:?} with id '{}'", ord, order_id);
    display_order(ord, &connection)
}

fn display_orders(mut ords: Vec<(Order, DeliveryMethod, Address)>, conn: &PgConnection)
    -> error_stack::Result<Vec<OrderToDisplay>, DbError> {
    Ok(ords.iter_mut().map(|ord| {
        let ordits = order_items
            .filter(parentorder.eq(&ord.0.id))
            .load::<OrderItem>(conn)
            .into_report()
            .change_context(DbError::NotFoundError);

        OrderToDisplay {
            id: ord.0.id,
            buyer_email: ord.0.buyer_email.clone(),
            order_date: ord.0.order_date,
            ship_to_address: AddressDto::from(ord.2.clone()),
            delivery_method: ord.1.short_name.clone(),
            shipping_price: ord.1.price,
            order_items: ordits.unwrap(),
            subtotal: ord.0.subtotal,
            status: ord.0.status.clone(),
            total: ord.0.total
        }
    }).collect::<Vec<OrderToDisplay>>())
}

fn display_order(ord: (Order, DeliveryMethod, Address), conn: &PgConnection)
                  -> error_stack::Result<OrderToDisplay, DbError> {
    let ordits = order_items
        .filter(parentorder.eq(&ord.0.id))
        .load::<OrderItem>(conn)
        .into_report()
        .change_context(DbError::NotFoundError);

    Ok(OrderToDisplay {
        id: ord.0.id,
        buyer_email: ord.0.buyer_email.clone(),
        order_date: ord.0.order_date,
        ship_to_address: AddressDto::from(ord.2.clone()),
        delivery_method: ord.1.short_name.clone(),
        shipping_price: ord.1.price,
        order_items: ordits.unwrap(),
        subtotal: ord.0.subtotal,
        status: ord.0.status.clone(),
        total: ord.0.total
    })
}