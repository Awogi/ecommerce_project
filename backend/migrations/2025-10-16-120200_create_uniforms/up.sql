-- create uniforms table
CREATE TABLE IF NOT EXISTS uniforms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    school_id INT NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    grade_id INT NOT NULL REFERENCES grades(id) ON DELETE CASCADE,
    category_id INT NOT NULL REFERENCES uniform_categories(id) ON DELETE CASCADE,
    size VARCHAR(10),
    price DOUBLE PRECISION NOT NULL,
    stock_quantity INT DEFAULT 0,
    image_url VARCHAR(255),
    created_at TIMESTAMP DEFAULT now()
    ,
    UNIQUE (school_id, grade_id, category_id, size)
);
