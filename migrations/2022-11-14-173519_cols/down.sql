-- This file should undo anything in `up.sql`
ALTER TABLE members
DROP COLUMN iss;

ALTER TABLE books
DROP COLUMN iss;
