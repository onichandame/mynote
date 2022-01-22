use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use model;
use sea_orm::entity::*;

use super::user::UserDTO;

#[derive(SimpleObject)]
#[graphql(name = "Note")]
pub struct NoteDTO {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub user_id: i32,
    pub title: String,
    pub content: String,
}

#[ComplexObject]
impl NoteDTO {
    async fn user(&self, ctx: &Context<'_>) -> Result<UserDTO> {
        let db = ctx.data::<model::Database>().unwrap();
        Ok(UserDTO::from(
            model::User::find_by_id(self.user_id)
                .one(db)
                .await?
                .ok_or("user not found")?,
        ))
    }
}

impl From<&model::note::Model> for NoteDTO {
    fn from(note: &model::note::Model) -> Self {
        Self {
            created_at: note.created_at.clone(),
            deleted_at: note.deleted_at.clone(),
            user_id: note.user_id.clone(),
            id: note.id.clone(),
            title: note.title.clone(),
            updated_at: note.updated_at.clone(),
            content: note.content.clone(),
        }
    }
}

impl From<model::note::Model> for NoteDTO {
    fn from(note: model::note::Model) -> Self {
        Self {
            created_at: note.created_at,
            deleted_at: note.deleted_at,
            user_id: note.user_id,
            id: note.id,
            title: note.title,
            updated_at: note.updated_at,
            content: note.content,
        }
    }
}
