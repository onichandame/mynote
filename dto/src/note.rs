use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use db::model;
use sqlx;

use super::user::UserDTO;

#[derive(SimpleObject)]
#[graphql(name = "Note")]
pub struct NoteDTO {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub user_id: i64,
    pub title: String,
    pub content: String,
}

#[ComplexObject]
impl NoteDTO {
    async fn user(&self, ctx: &Context<'_>) -> Result<UserDTO> {
        let pool = ctx.data::<db::ConnectionPool>().unwrap();
        Ok(
            sqlx::query_as::<_, model::User>("SELECT * FROM users WHERE id = ?")
                .bind(self.user_id)
                .fetch_one(pool)
                .await?
                .into(),
        )
    }
}

impl From<&model::Note> for NoteDTO {
    fn from(note: &model::Note) -> Self {
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

impl From<model::Note> for NoteDTO {
    fn from(note: model::Note) -> Self {
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
