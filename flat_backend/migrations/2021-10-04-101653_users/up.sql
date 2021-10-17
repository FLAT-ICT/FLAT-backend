-- Your SQL goes here
CREATE TABLE users (
    id INT NOT NULL AUTO_INCREMENT,
    user_id TEXT NOT NULL,
    user_name TEXT NOT NULL,
    status INT NOT NULL DEFAULT 0,
    beacon TEXT ,
    icon_path TEXT NOT NULL ,
    hashed_password TEXT NOT NULL,
    PRIMARY KEY(id)
);
