-- Your SQL goes here
CREATE TABLE friends (
    acctive TEXT NOT NULL,
    pussive TEXT NOT NULl,
    delete_flag BOOLEAN NOT NULL default 0,
    PRIMARY KEY(acctive(20), pussive(20))
);