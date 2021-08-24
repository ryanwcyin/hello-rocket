-- Your SQL goes here
CREATE TABLE POST(
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    publish_date DATE,
    content TEXT
)