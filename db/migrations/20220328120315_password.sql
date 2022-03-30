-- Add migration script here
CREATE table password_groups(
    id integer not null primary key,
    uuid text not null unique default ( lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))),
    lamport_clock integer not null default 0,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    parent_id integer,
    title text not null,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(parent_id) REFERENCES password_groups(id),
    UNIQUE(user_id, parent_id, title)
);
CREATE UNIQUE INDEX password_groups_user_id_title ON password_groups(user_id, title) WHERE parent_id IS NULL;
CREATE table passwords(
    id integer not null primary key,
    uuid text not null unique default ( lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))),
    lamport_clock integer not null default 0,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    group_id integer,
    title text not null,
    password text not null,
    url text,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(group_id) REFERENCES password_groups(id),
    UNIQUE(user_id, group_id, title)
);
CREATE UNIQUE INDEX passwords_user_id_title ON passwords(user_id, title) WHERE group_id IS NULL;