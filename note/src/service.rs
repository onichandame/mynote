use std::error::Error;

use model;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set, Unchanged,
};

use crate::error::NoteError;

pub async fn create_note(
    db: &model::Database,
    mut note: model::note::ActiveModel,
    userid: i32,
) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
    note.user_id = Set(userid);
    Ok(note.insert(db).await?)
}

pub async fn update_note(
    db: &model::Database,
    id: i32,
    mut update: model::note::ActiveModel,
) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
    update.id = Unchanged(id);
    Ok(update.update(db).await?)
}

pub async fn delete_note(
    db: &model::Database,
    id: i32,
    userid: Option<i32>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut query = model::Note::find_by_id(id);
    match userid {
        Some(userid) => query = query.filter(model::note::Column::UserId.eq(userid)),
        None => {}
    }
    let note = query.one(db).await?.ok_or(NoteError::NoteNotFound)?;
    note.delete(db).await?;
    Ok(())
}

pub async fn find_note(
    db: &model::Database,
    id: i32,
    userid: Option<i32>,
) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
    let mut query = model::Note::find_by_id(id);
    match userid {
        Some(userid) => query = query.filter(model::note::Column::UserId.eq(userid)),
        None => {}
    };
    Ok(query.one(db).await?.ok_or(NoteError::NoteNotFound)?)
}

pub async fn find_notes(
    db: &model::Database,
    userid: Option<i32>,
) -> Result<Vec<model::note::Model>, Box<dyn Error + Send + Sync>> {
    let mut query = model::Note::find();
    match userid {
        Some(userid) => query = query.filter(model::note::Column::UserId.eq(userid)),
        None => {}
    };
    Ok(query.all(db).await?)
}
