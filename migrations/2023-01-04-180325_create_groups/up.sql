CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(25) UNIQUE NOT NULL,
    closed BOOLEAN NOT NULL DEFAULT FALSE
)