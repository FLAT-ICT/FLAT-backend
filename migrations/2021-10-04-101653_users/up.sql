-- Your SQL goes here
CREATE FUNCTION set_update_time() RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'UPDATE') THEN
        NEW.updated_at = now();
        RETURN NEW;
    END IF;
END;
$$ language plpgsql;

CREATE TABLE users (
    id SERIAL,
    name VARCHAR(10) NOT NULL UNIQUE,
    status INT NOT NULL DEFAULT 0,
    spot TEXT,
    icon_path TEXT NOT NULL,
    salt BYTEA NOT NULL,
    hash BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    logged_in_at TIMESTAMP NULL DEFAULT NULL,
    PRIMARY KEY(id)
);

CREATE TRIGGER trg_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE set_update_time();