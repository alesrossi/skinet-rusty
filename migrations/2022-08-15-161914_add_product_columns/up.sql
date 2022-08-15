-- Your SQL goes here
ALTER TABLE products
    ADD COLUMN description VARCHAR(255) NOT NULL DEFAULT '',
    ADD COLUMN pictureUrl VARCHAR(255) NOT NULL,
    ADD COLUMN productBrand INTEGER NOT NULL,
    ADD COLUMN productType INTEGER NOT NULL,
    ADD CONSTRAINT fk_brand
        FOREIGN KEY (productBrand)
            REFERENCES product_brands(id),
    ADD CONSTRAINT fk_type
        FOREIGN KEY (productType)
            REFERENCES product_types(id),
    ALTER COLUMN cost TYPE REAL;