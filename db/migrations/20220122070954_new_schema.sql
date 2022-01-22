-- Add migration script here
ALTER TABLE users RENAME TO _users;
CREATE table users(
    id integer not null primary key,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    name text not null unique,
    password text not null,
    email text unique,
    avatar text
);
INSERT INTO users ( id, created_at, updated_at, deleted_at, name, password, email, avatar)
    SELECT id, created_at, updated_at, deleted_at, name, password, email, avatar
    FROM _users;

ALTER TABLE session_keys RENAME TO _session_keys;
CREATE table session_keys(
    id integer not null primary key,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    key text not null
);
INSERT INTO session_keys ( id, created_at, updated_at, deleted_at, key)
    SELECT id, created_at, updated_at, deleted_at, key
    FROM _session_keys;

ALTER TABLE notes RENAME TO _notes;
CREATE table notes(
    id integer not null primary key,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    title text not null,
    content text not null,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
INSERT INTO notes ( id, created_at, updated_at, deleted_at, user_id, title, content)
    SELECT id, created_at, updated_at, deleted_at, user_id, title, content
    FROM _notes;

drop table _notes;
drop table _session_keys;
drop table _users;