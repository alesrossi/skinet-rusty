CREATE TABLE addresses (
                           id SERIAL PRIMARY KEY,
                           firstName VARCHAR NOT NULL,
                           lastName VARCHAR NOT NULL,
                           street VARCHAR NOT NULL,
                           city VARCHAR NOT NULL,
                           country VARCHAR NOT NULL,
                           postalCode VARCHAR NOT NULL
);

CREATE TABLE app_users (
    id SERIAL PRIMARY KEY,
    displayName VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    address INTEGER,
    CONSTRAINT fk_address
        FOREIGN KEY (address)
            REFERENCES addresses(id)
);

