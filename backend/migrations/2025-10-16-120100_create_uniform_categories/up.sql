-- create uniform_categories table
CREATE TABLE IF NOT EXISTS uniform_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);
