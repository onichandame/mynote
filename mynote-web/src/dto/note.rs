use async_graphql::{InputObject, MaybeUndefined, SimpleObject};
use model;

use super::DateTimeFilter;

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

#[derive(InputObject)]
pub struct NoteFilterDTO {
    deleted_at: Option<DateTimeFilter>,
}

#[derive(InputObject)]
pub struct NoteInputDTO {
    pub title: String,
    pub content: String,
}

#[derive(InputObject)]
pub struct NoteUpdateDTO {
    pub title: Option<String>,
    pub content: Option<String>,
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

impl Into<Filter> for NoteFilterDTO {
    fn into(self) -> Filter {
        Filter {
            ..Default::default()
        }
    }
}
