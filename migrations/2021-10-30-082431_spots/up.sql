-- Your SQL goes here
CREATE TABLE spots (
    ja_spot TEXT NOT NULL,
    en_spot TEXT NOT NULL,
    region_identifire TEXT NOT NULL,
    from_date TIMESTAMP NOT NULL,
    to_date TIMESTAMP NULL DEFAULT NULL,
    major INTEGER NOT NULL,
    minor INTEGER NOT NULL,
    PRIMARY KEY (major, minor)
);