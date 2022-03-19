-- Add migration script here
ALTER TABLE notes RENAME TO _notes;
CREATE table notes(
    id integer not null primary key,
    uuid text not null unique default (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-' || substr('89ab',abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))),2) || '-' || lower(hex(randomblob(6)))),
    lamport_clock integer not null default 0,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    title text not null,
    content text not null,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
INSERT INTO notes (id, created_at, updated_at, deleted_at, user_id, title, content)
    SELECT id, created_at, updated_at, deleted_at, user_id, title, content
    FROM _notes;

drop table _notes;
