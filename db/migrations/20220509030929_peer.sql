-- Add migration script here
CREATE table peers(
    id integer not null primary key,
    uuid text not null unique default ( lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6)))),
    created_at datetime not null default current_timestamp,
    updated_at datetime,
    deleted_at datetime,

    user_id integer not null,
    password_id integer not null,
    title text not null,
    auto_sync boolean not null default false,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(password_id) REFERENCES passwords(id)
);
