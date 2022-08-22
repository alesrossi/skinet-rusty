table! {
    addresses (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        street -> Varchar,
        city -> Varchar,
        country -> Varchar,
        postalcode -> Varchar,
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
        price -> Float4,
        description -> Varchar,
        pictureurl -> Varchar,
        productbrand -> Int4,
        producttype -> Int4,
    }
}

joinable!(app_users -> addresses (address));
joinable!(products -> product_brands (productbrand));
joinable!(products -> product_types (producttype));

allow_tables_to_appear_in_same_query!(
    addresses,
    app_users,
    product_brands,
    product_types,
    products,
);
