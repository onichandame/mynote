-- Add up migration script here
CREATE table users(
    id integer primary key,
    name text not null unique,
    password text not null,
    email text unique,
    created_at datetime default current_timestamp,
    updated_at datetime,
    deleted_at datetime
)
