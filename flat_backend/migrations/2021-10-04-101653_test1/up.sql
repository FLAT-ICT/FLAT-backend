-- Your SQL goes here
CREATE TABLE users (
    id INT PRIMARY KEY,
    user_id TEXT NOT NULL,
    user_name TEXT NOT NULL,
    status INT,
    beacon TEXT,
    icon_path TEXT,
    hashed_password TEXT
);
CREATE TABLE friends (
    id INT PRIMARY KEY,
    acctive TEXT NOT NULL,
    pussive TEXT NOT NULl,
    delete_flag BOOLEAN DEFAULT 'false'
);