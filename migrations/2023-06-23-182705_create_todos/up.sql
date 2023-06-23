-- Your SQL goes here

CREATE TABLE todos(
    id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    created_on TEXT,
    deadline TEXT 
);
