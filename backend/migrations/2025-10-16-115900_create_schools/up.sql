-- create schools table
CREATE TABLE IF NOT EXISTS schools (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    address VARCHAR(255),
    contact_number VARCHAR(20),
    created_at TIMESTAMP DEFAULT now()
);
