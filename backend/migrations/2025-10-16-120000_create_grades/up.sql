-- create grades table
CREATE TABLE IF NOT EXISTS grades (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    school_id INT NOT NULL REFERENCES schools(id) ON DELETE CASCADE,
    UNIQUE (name, school_id)
);
