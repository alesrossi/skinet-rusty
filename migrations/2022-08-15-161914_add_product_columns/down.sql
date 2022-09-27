-- This file should undo anything in `up.sql`
ALTER TABLE products
    DROP COLUMN description,
    DROP COLUMN pictureUrl,
    DROP COLUMN productBrand,
    DROP COLUMN productType,

    ALTER COLUMN price TYPE INTEGER;