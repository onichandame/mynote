-- Add up migration script here
CREATE table session_keys(
    id integer primary key,
    created_at datetime default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    key text not null
)

