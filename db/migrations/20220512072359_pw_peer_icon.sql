-- Add migration script here
ALTER TABLE passwords RENAME TO _passwords;
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
    icon text,
    password text not null,
    username text,
    url text,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(group_id) REFERENCES password_groups(id)
);

INSERT INTO passwords (id, uuid, created_at, updated_at, deleted_at, user_id, group_id, title, password, username, url)
    SELECT id, uuid, created_at, updated_at, deleted_at, user_id, group_id, title, password, username, url
    FROM _passwords;

ALTER TABLE peers RENAME TO _peers;
CREATE table peers(
    id integer not null primary key,
    uuid text not null unique default ( lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))),
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    password_id integer not null,
    title text not null,
    icon text,
    auto_sync boolean not null default false,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(password_id) REFERENCES passwords(id)
);

INSERT INTO peers (id, uuid, created_at, updated_at, deleted_at, user_id, password_id, title, auto_sync)
    SELECT id, uuid, created_at, updated_at, deleted_at, user_id, password_id, title, auto_sync
    FROM _peers;

drop table _peers;
drop table _passwords;