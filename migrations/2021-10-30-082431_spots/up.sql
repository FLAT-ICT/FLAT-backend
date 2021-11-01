-- Your SQL goes here
CREATE TABLE spots (
    name_ja TEXT NOT NULL,
    name_en TEXT NOT NULL,
    region_identifier TEXT NOT NULL,
    available_term_from TIMESTAMP NOT NULL,
    available_term_to TIMESTAMP NULL DEFAULT NULL,
    major INTEGER NOT NULL,
    minor INTEGER NOT NULL,
    note TEXT,
    PRIMARY KEY (major, minor)
);