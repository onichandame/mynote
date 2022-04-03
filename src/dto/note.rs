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

#[derive(InputObject, Default)]
#[graphql(name = "NoteFilter")]
pub struct NoteFilterDTO {
    deleted_at: Option<DateTimeFilter>,
}

#[derive(Enum, Clone, Copy, Eq, PartialEq)]
#[graphql(name = "NoteField")]
pub enum NoteFieldDTO {
    CreatedAt,
}

#[derive(InputObject)]
#[graphql(name = "NoteSorting")]
pub struct NoteSortingDTO {
    field: NoteFieldDTO,
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

impl From<model::note::Model> for NoteDTO {
    fn from(note: model::note::Model) -> Self {
        Self {
            created_at: note.created_at,
            deleted_at: note.deleted_at,
            user_id: note.user_id,
            id: note.id,
            uuid: note.uuid,
            lamport_clock: note.lamport_clock,
            title: note.title,
            updated_at: note.updated_at,
            content: note.content,
        }
    }
}

impl Into<NoteFilter> for NoteFilterDTO {
    fn into(self) -> NoteFilter {
        NoteFilter {
            deleted_at: self.deleted_at.map(|v| v.into_filter()),
            ..Default::default()
        }
    }
}

impl Into<model::note::Column> for NoteFieldDTO {
    fn into(self) -> model::note::Column {
        match self {
            Self::CreatedAt => model::note::Column::CreatedAt,
        }
    }
}

impl Into<NoteSorting> for NoteSortingDTO {
    fn into(self) -> NoteSorting {
        NoteSorting {
            field: self.field.into(),
            direction: self.direction.into(),
        }
    }
}

impl Into<model::note::Insert> for NoteInputDTO {
    fn into(self) -> model::note::Insert {
        model::note::Insert {
            title: self.title,
            content: self.content,
            ..Default::default()
        }
    }
}

impl Into<model::note::Update> for NoteUpdateDTO {
    fn into(self) -> model::note::Update {
        model::note::Update {
            title: self.title,
            content: self.content,
            ..Default::default()
        }
    }
}
