-- Your SQL goes here
-- TODO add references to books borrowed
CREATE TABLE members (
id SERIAL PRIMARY KEY,
fst_name VARCHAR(256) NOT NULL,
lst_name VARCHAR(256) NOT NULL,
dob DATE NOT NULL)

