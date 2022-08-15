table! {
    product_brands (id) {
        id -> Int4,
        name -> Varchar,
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
        cost -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    product_brands,
    product_types,
    products,
);
