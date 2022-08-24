-- Your SQL goes here
CREATE TABLE delivery_methods (
    id SERIAL PRIMARY KEY,
    shortName VARCHAR NOT NULL,
    deliveryTime VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    price REAL NOT NULL
);