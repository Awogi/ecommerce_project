-- create payments table
CREATE TABLE IF NOT EXISTS payments (
    id SERIAL PRIMARY KEY,
    order_id INT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    payment_method VARCHAR(50) NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',
    paid_at TIMESTAMP
);
