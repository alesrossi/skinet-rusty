-- Your SQL goes here
ALTER TABLE orders
    DROP COLUMN status;

CREATE TYPE order_status AS ENUM ('pending', 'paymentreceived', 'paymentfailed');
ALTER TABLE orders
    ADD COLUMN status order_status NOT NULL DEFAULT 'pending';