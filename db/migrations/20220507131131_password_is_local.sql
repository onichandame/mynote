-- Add migration script here
ALTER TABLE passwords RENAME TO _passwords;
drop index passwords_user_id_title;
drop index passwords_user_id_group_id_title;
CREATE table passwords(
    id integer not null primary key,
    uuid text not null unique default ( lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))),
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    group_id integer,
    is_local boolean not null default false,
    title text not null,
    password text not null,
    username text,
    email text,
    url text,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(group_id) REFERENCES password_groups(id)
);

INSERT INTO passwords (id, uuid, created_at, updated_at, deleted_at, user_id, group_id, title, password, username, email, url)
    SELECT id, uuid, created_at, updated_at, deleted_at, user_id, group_id, title, password, username, email, url
    FROM _passwords;

drop table _passwords;

