-- Your SQL goes here
CREATE TABLE alerts (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR NOT NULL,
    basis VARCHAR NOT NULL,
    ma_length INTEGER,
    value DOUBLE PRECISION NOT NULL,
    direction VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'pending'
);
