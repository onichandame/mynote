use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Name,
    Email,
    Avatar,
}

#[derive(Iden)]
pub enum Credential {
    Table,
    Id,
    CreatedAt,
    UserId,
    Password,
}

#[derive(Iden)]
pub enum Note {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    AuthorId,
    Title,
    Content,
}

#[derive(Iden)]
pub enum Password {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    UserId,
    Title,
    Password,
    Icon,
    Url,
    UserName,
}

#[derive(Iden)]
pub enum SessionKey {
    Table,
    Id,
    CreatedAt,
    Key,
}
