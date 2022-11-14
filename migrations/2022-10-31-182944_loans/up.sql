-- Creates table in which issues of books to members will be stored
CREATE TABLE issues (
id SERIAL PRIMARY KEY,
doi DATE NOT NULL DEFAULT CURRENT_DATE,
due DATE NOT NULL,
bk_id INTEGER REFERENCES books(id),
mbr_id INTEGER REFERENCES members(id))
