use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Memo {
    Table,
    Id,
    Uuid,
    Revision,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Title,
    Content,
    Weight, // unsigned int. 0 at top
}
