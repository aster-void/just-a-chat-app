INSERT INTO workspaces (id, name, public) VALUES
(1, 'new public workspace', true),
(2, 'private workspace', false);


INSERT INTO roles (name, workspace_id) VALUES
('admin role', 1),
('admin role', 2),
('public ws role', 1),
('priv ws role', 2);

INSERT INTO channels (id, name, workspace_id, is_dm) VALUES
(1, 'pub chan 1', 1, false),
(2, 'priv chan 1', 2, false),
(3, 'pub chan 2', 1, false),
(4, 'priv chan 2', 2, false);

INSERT INTO users (id, name, bcrypt_pass) VALUES
-- username's  password is 'password' (without the quotes)
(1, 'username', '$2b$12$rgCM22eVfRbKia8K4ba0EuU6wDN8f/3H2QmT4xTyk2F/CHp/TLHXC'),
(2, 'another user', 'cannot not login');

INSERT INTO messages
(content, posted_at, posted_chan, posted_workspace, posted_by)
VALUES
('hello', 1000, 1, 1, 1);

-- user -> belongs -> workspace
INSERT INTO belongs (user_id,workspace_id) VALUES 
(1, 1),
(1, 2),
(2, 1);

-- user -> member_of -> channel
INSERT INTO member_of (user_id,channel_id) VALUES 
(1, 1),
(1, 2),
(1, 3),
(2, 1),
(2, 2);
