-- Your SQL goes here
ALTER TABLE addresses
    RENAME COLUMN country TO state;

ALTER TABLE addresses
    RENAME COLUMN postalcode TO zipcode;