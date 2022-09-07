-- Your SQL goes here
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    orderDate timestamp NOT NULL,
    address INTEGER NOT NULL,
    deliveryMethod INTEGER NOT NULL,
    subtotal REAL NOT NULL,
    total REAL NOT NULL,
    paymentIntentId VARCHAR NOT NULL,
    CONSTRAINT fk_address
        FOREIGN KEY (address)
            REFERENCES addresses,
    CONSTRAINT fk_delivery_methods
        FOREIGN KEY (deliveryMethod)
            REFERENCES delivery_methods
);

CREATE TABLE product_order_items (
    id SERIAL PRIMARY KEY,
    productId INTEGER NOT NULL,
    productName VARCHAR NOT NULL,
    pictureUrl VARCHAR NOT NULL

);

CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    productItemOrdered INTEGER NOT NULL,
    price REAL NOT NULL,
    quantity INTEGER NOT NULL,
    parentOrder INTEGER NOT NULL,
    CONSTRAINT fk_parent_orders
        FOREIGN KEY (parentOrder)
            REFERENCES orders,
    CONSTRAINT fk_product_item_ordered
        FOREIGN KEY (productItemOrdered)
            REFERENCES product_order_items
)