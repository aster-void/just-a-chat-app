-- Add up migration script here
CREATE TABLE workspaces (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    public BOOLEAN NOT NULL
);

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    workspace_id int4 NOT NULL REFERENCES workspaces(id),
    UNIQUE (workspace_id, name)
);

CREATE TABLE channels (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    workspace_id int4 NOT NULL REFERENCES workspaces(id),
    UNIQUE (workspace_id, name)
);

CREATE TABLE users (
    id SERIAL UNIQUE PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    bcrypt_pass VARCHAR(256) NOT NULL
);

-- user -> belongs -> workspace
CREATE TABLE belongs (
    user_id int4 NOT NULL REFERENCES users(id),
    workspace_id int4 NOT NULL REFERENCES workspaces(id),
    PRIMARY KEY (workspace_id, user_id)
);
