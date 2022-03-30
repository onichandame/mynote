use async_graphql::{Enum, InputObject, SimpleObject};
use model;
use note::{NoteFilter, NoteSorting};

use super::{DateTimeFilter, IntoFilter, SortDirectionDTO};

#[derive(SimpleObject)]
#[graphql(name = "Note")]
pub struct NoteDTO {
    pub id: i32,
    pub uuid: String,
    pub lamport_clock: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub user_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(InputObject)]
#[graphql(name = "NoteFilter")]
pub struct NoteFilterDTO {
    deleted_at: Option<DateTimeFilter>,
}

#[derive(Enum, Clone, Copy, Eq, PartialEq)]
#[graphql(name = "NoteSortingField")]
pub enum NoteSortingFieldDTO {
    CreatedAt,
}

#[derive(InputObject)]
#[graphql(name = "NoteSorting")]
pub struct NoteSortingDTO {
    field: NoteSortingFieldDTO,
    direction: SortDirectionDTO,
}

#[derive(InputObject)]
#[graphql(name = "NoteInput")]
pub struct NoteInputDTO {
    pub title: String,
    pub content: String,
}

#[derive(InputObject)]
pub struct NoteUpdateDTO {
    pub title: Option<String>,
    pub content: Option<String>,
}

pub trait IntoNoteFilter {
    fn into_note_filter(&self) -> NoteFilter;
}

impl From<&model::note::Model> for NoteDTO {
    fn from(note: &model::note::Model) -> Self {
        Self {
            created_at: note.created_at.clone(),
            deleted_at: note.deleted_at.clone(),
            user_id: note.user_id.clone(),
            id: note.id.clone(),
            uuid: note.uuid.clone(),
            lamport_clock: note.lamport_clock.clone(),
            title: note.title.clone(),
            updated_at: note.updated_at.clone(),
            content: note.content.clone(),
        }
    }
}

impl IntoNoteFilter for NoteFilterDTO {
    fn into_note_filter(&self) -> NoteFilter {
        NoteFilter {
            deleted_at: self.deleted_at.clone().map(|v| v.into_filter()),
            ..Default::default()
        }
    }
}

impl Into<NoteSorting> for NoteSortingDTO {
    fn into(self) -> NoteSorting {
        NoteSorting {
            field: match self.field {
                NoteSortingFieldDTO::CreatedAt => model::note::Column::CreatedAt,
            },
            direction: self.direction.into(),
        }
    }
}
