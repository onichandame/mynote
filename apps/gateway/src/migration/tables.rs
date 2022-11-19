use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Invitation {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    InviterId,
    Key,
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Name,
    Email,
    Avatar,
    InvitationId,
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
pub enum Memo {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    AuthorId,
    Content,
    Weight,
}

#[derive(Iden)]
pub enum SessionKey {
    Table,
    Id,
    CreatedAt,
    Key,
}
