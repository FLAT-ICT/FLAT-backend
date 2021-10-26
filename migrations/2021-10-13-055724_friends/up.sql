-- Your SQL goes here
CREATE TABLE friends (
    acctive INT NOT NULL,
    pussive INT NOT NULl,
    delete_flag BOOLEAN NOT NULL default 0,
    PRIMARY KEY(acctive, pussive)
);