-- Your SQL goes here
-- TODO Need to add link to loan table for due date, I think
CREATE TYPE status AS ENUM ('LOST','BORROWED','AVAILABLE');
CREATE TABLE books (
id SERIAL PRIMARY KEY,
title VARCHAR(256) NOT NULL,
auth_fst VARCHAR(256) NOT NULL,
auth_lst VARCHAR(256) NOT NULL,
curr_status status NOT NULL,
isbn CHAR(13) NOT NULL)
