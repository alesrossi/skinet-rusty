-- This file should undo anything in `up.sql`
ALTER TABLE orders
    DROP COLUMN status;
ALTER TABLE orders
    ADD COLUMN status VARCHAR NOT NULL DEFAULT 'pending';
DROP TYPE  order_status;
