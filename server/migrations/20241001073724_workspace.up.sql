-- Add up migration script here
CREATE TABLE workspaces (
    id SERIAL UNIQUE PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);
