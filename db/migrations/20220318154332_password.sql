-- Add migration script here
CREATE table master_passwords(
    id integer not null primary key,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    name text not null unique,
    password text not null,
    user_id integer not null,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
CREATE table passwords(
    id integer not null primary key,
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    name text not null unique,
    password text not null
);