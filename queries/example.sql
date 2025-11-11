CREATE TABLE users (
    user_id INT,
    username VARCHAR(100) NOT NULL,
    email TEXT,
    is_active BOOLEAN,
    created_at DATE
);