-- This file should undo anything in `up.sql`
ALTER TABLE addresses
    RENAME COLUMN state TO country;

ALTER TABLE addresses
    RENAME COLUMN zipcode TO postalcode;