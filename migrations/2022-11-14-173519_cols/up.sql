-- Adds columns to books and members databases
-- referring to most recent active issue pertaining to said
-- book or member

ALTER TABLE members
ADD COLUMN iss INTEGER REFERENCES issues(id);

ALTER TABLE books
ADD COLUMN iss INTEGER REFERENCES issues(id);
