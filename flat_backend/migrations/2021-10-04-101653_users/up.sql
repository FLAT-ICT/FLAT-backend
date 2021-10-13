-- Your SQL goes here
CREATE TABLE users (
    id INT NOT NULL AUTO_INCREMENT,
    user_id TEXT NOT NULL,
    user_name TEXT NOT NULL,
    status INT NOT NULL DEFAULT 0,
    beacon TEXT NOT NULL DEFAULT "",
    icon_path TEXT DEFAULT "",
    hashed_password TEXT NOT NULL,
    PRIMARY KEY(id)
);