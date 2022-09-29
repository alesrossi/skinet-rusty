table! {
    addresses (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        street -> Varchar,
        city -> Varchar,
        state -> Varchar,
        zipcode -> Varchar,
    }
}

table! {
    app_users (id) {
        id -> Int4,
        displayname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        address -> Nullable<Int4>,
    }
}

table! {
    delivery_methods (id) {
        id -> Int4,
        shortname -> Varchar,
        deliverytime -> Varchar,
        description -> Varchar,
        price -> Float4,
    }
}

table! {
    order_items (id) {
        id -> Int4,
        productitemordered -> Int4,
        price -> Float4,
        quantity -> Int4,
        parentorder -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::models::exports::*;


    orders (id) {
        id -> Int4,
        buyeremail -> Varchar,
        orderdate -> Timestamp,
        address -> Int4,
        deliverymethod -> Int4,
        subtotal -> Float4,
        total -> Float4,
        status -> OrderStatus,
        paymentintentid -> Varchar,
    }
}

table! {
    product_brands (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    product_order_items (id) {
        id -> Int4,
        productid -> Int4,
        productname -> Varchar,
        pictureurl -> Varchar,
    }
}

table! {
    product_types (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        price -> Float4,
        description -> Varchar,
        pictureurl -> Varchar,
        productbrand -> Int4,
        producttype -> Int4,
    }
}

joinable!(order_items -> orders (parentorder));
joinable!(order_items -> product_order_items (productitemordered));
joinable!(orders -> delivery_methods (deliverymethod));
joinable!(products -> product_brands (productbrand));
joinable!(products -> product_types (producttype));

allow_tables_to_appear_in_same_query!(
    addresses,
    app_users,
    delivery_methods,
    order_items,
    orders,
    product_brands,
    product_order_items,
    product_types,
    products,
);
