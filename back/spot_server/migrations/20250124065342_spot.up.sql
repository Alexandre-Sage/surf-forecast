-- Add up migration script here
CREATE TABLE IF NOT EXISTS spots (
    id uuid PRIMARY KEY UNIQUE NOT NULL,
    name varchar(100) UNIQUE NOT NULL,
    windguru_id integer UNIQUE,
    latitude decimal,
    longitude decimal,
    created_at timestamp NOT NULL
);
