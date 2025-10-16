-- create orders table
CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    total_price DOUBLE PRECISION NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT now()
);

-- create order_items table
CREATE TABLE IF NOT EXISTS order_items (
    id SERIAL PRIMARY KEY,
    order_id INT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    uniform_id INT NOT NULL REFERENCES uniforms(id) ON DELETE CASCADE,
    quantity INT NOT NULL DEFAULT 1,
    price DOUBLE PRECISION NOT NULL
);
