-- Add up migration script here
CREATE TABLE belongs (
    workspace_id int4,
    user_id int4,
    FOREIGN KEY (workspace_id) REFERENCES workspaces (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (workspace_id, user_id)
);
