-- Migration: create users table
CREATE TABLE IF NOT EXISTS users (
	id SERIAL PRIMARY KEY,
	full_name VARCHAR(100) NOT NULL,
	email VARCHAR(100) NOT NULL UNIQUE,
	password_hash VARCHAR(255) NOT NULL,
	role VARCHAR(20) NOT NULL DEFAULT 'student', -- possible values: student, parent, admin
	school_id INT NULL REFERENCES schools(id) ON DELETE SET NULL,
	created_at TIMESTAMP NOT NULL DEFAULT now()
);
