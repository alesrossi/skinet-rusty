-- Your SQL goes here

CREATE TABLE product_brands (
                          id SERIAL PRIMARY KEY,
                          name VARCHAR NOT NULL
);

CREATE TABLE product_types (
                        id SERIAL PRIMARY KEY,
                        name VARCHAR NOT NULL
);