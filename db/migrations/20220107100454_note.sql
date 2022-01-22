-- Add up migration script here
CREATE table notes(
    id integer primary key,
    created_at datetime default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    title text not null,
    content text not null,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

