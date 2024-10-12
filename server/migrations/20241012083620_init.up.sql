-- Add up migration script here
CREATE TABLE workspaces (
    id SERIAL UNIQUE PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE users (
    id SERIAL UNIQUE PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    bcrypt_pass VARCHAR(256) NOT NULL
);

-- user -> belongs -> workspace
CREATE TABLE belongs (
    user_id int4 REFERENCES users (id),
    workspace_id int4 REFERENCES workspaces (id),
    PRIMARY KEY (workspace_id, user_id)
);
