-- Add migration script here
PRAGMA foreign_keys=off;
-- users
CREATE table _users(
    id integer not null primary key,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    name text not null unique,
    password text not null,
    email text unique,
    avatar text
);
INSERT INTO _users ( id, created_at, updated_at, deleted_at, name, password, email, avatar)
    SELECT id, created_at, updated_at, deleted_at, name, password, email, avatar
    FROM users;
DROP TABLE users;
ALTER TABLE _users RENAME TO users;

-- session keys
CREATE table _session_keys(
    id integer not null primary key,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    key text not null
);
INSERT INTO _session_keys ( id, created_at, updated_at, deleted_at, key)
    SELECT id, created_at, updated_at, deleted_at, key
    FROM session_keys;
DROP TABLE session_keys;
ALTER TABLE _session_keys RENAME TO session_keys;

-- notes
CREATE table _notes(
    id integer not null primary key,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    title text not null,
    content text not null,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
INSERT INTO _notes ( id, created_at, updated_at, deleted_at, user_id, title, content)
    SELECT id, created_at, updated_at, deleted_at, user_id, title, content
    FROM notes;
DROP TABLE notes;
ALTER TABLE _notes RENAME TO notes;

PRAGMA foreign_keys=on;