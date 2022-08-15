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
        cost -> Float4,
        description -> Varchar,
        pictureurl -> Varchar,
        productbrand -> Int4,
        producttype -> Int4,
    }
}

joinable!(products -> product_brands (productbrand));
joinable!(products -> product_types (producttype));

allow_tables_to_appear_in_same_query!(
    product_brands,
    product_types,
    products,
);
