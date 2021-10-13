-- Your SQL goes here
CREATE TABLE friends (
    id INT AUTO_INCREMENT,
    acctive TEXT NOT NULL,
    pussive TEXT NOT NULl,
    delete_flag BOOLEAN NOT NULL default 0,
    PRIMARY KEY(id)
);