CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    login VARCHAR(25) UNIQUE NOT NULL
)