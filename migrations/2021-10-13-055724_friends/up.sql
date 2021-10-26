-- Your SQL goes here
CREATE TABLE friends (
    acctive INT NOT NULL,
    pussive INT NOT NULl,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    blocked_at TIMESTAMP NULL DEFAULT NULL,
    PRIMARY KEY(acctive, pussive)
);