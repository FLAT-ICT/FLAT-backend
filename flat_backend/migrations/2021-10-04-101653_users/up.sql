-- Your SQL goes here
CREATE TABLE users (
    id INT AUTO_INCREMENT,
    user_id TEXT NOT NULL,
    user_name TEXT NOT NULL,
    status INT,
    beacon TEXT,
    icon_path TEXT,
    hashed_password TEXT,
    PRIMARY KEY(id)
);