CREATE TABLE santa (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    present_id INTEGER NOT NULL ,
    group_id INTEGER NOT NULL
)