-- Add up migration script here
ALTER TABLE users ADD COLUMN
    bcrypt_pass VARCHAR(256) NOT NULL
;
