-- Your SQL goes here
CREATE TABLE friends (
    active INT NOT NULL,
    passive INT NOT NULl,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    blocked_at TIMESTAMP NULL DEFAULT NULL,
    PRIMARY KEY(active, passive)
);