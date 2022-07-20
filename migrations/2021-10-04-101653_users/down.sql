-- This file should undo anything in `up.sql`
DROP TRIGGER update_tri ON users;
DROP FUNCTION set_update_time();
DROP TABLE users;