-- Add up migration script here
CREATE TABLE workspaces (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    public BOOLEAN NOT NULL
);

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    workspace_id int4 NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    UNIQUE (workspace_id, name)
);

CREATE TABLE channels (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    workspace_id int4 NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    is_dm boolean NOT NULL,
    UNIQUE (workspace_id, name)
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    bcrypt_pass VARCHAR(256) NOT NULL
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    posted_at int8 NOT NULL,
    posted_chan int4 NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    posted_workspace int4 NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    posted_by int4 NOT NULL REFERENCES users(id)
);

-- user -> belongs -> workspace
CREATE TABLE belongs (
    user_id int4 NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    workspace_id int4 NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    PRIMARY KEY (workspace_id, user_id)
);

-- user -> member_of -> channel
CREATE TABLE member_of (
    user_id int4 NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    channel_id int4 NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, channel_id)
);
