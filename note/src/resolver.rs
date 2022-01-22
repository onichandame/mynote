use async_graphql::{Context, Object, Result};
use dto::NoteDTO;
use model;
use sea_orm::{NotSet, Set};

use crate::{
    dto::{NoteInputDTO, NoteUpdateDTO},
    service,
};

#[derive(Default)]
pub struct NoteQuery;
#[derive(Default)]
pub struct NoteMutation;

#[Object]
impl NoteQuery {
    #[graphql(guard = "auth::LoginRequired::new()")]
    async fn list_notes<'a>(&self, ctx: &Context<'a>) -> Result<Vec<NoteDTO>> {
        let db = ctx.data::<model::Database>().unwrap();
        let user = auth::get_user_from_ctx(ctx).await?;
        Ok(service::find_notes(db, Some(user.id))
            .await?
            .iter()
            .map(|note| NoteDTO::from(note))
            .collect::<Vec<_>>())
    }

    #[graphql(guard = "auth::LoginRequired::new()")]
    async fn get_note<'a>(&self, ctx: &Context<'a>, id: i32) -> Result<NoteDTO> {
        let db = ctx.data::<model::Database>().unwrap();
        let user = auth::get_user_from_ctx(ctx).await?;
        Ok(NoteDTO::from(
            service::find_note(db, id, Some(user.id)).await?,
        ))
    }
}

#[Object]
impl NoteMutation {
    #[graphql(guard = "auth::LoginRequired::new()")]
    async fn create_note(&self, ctx: &Context<'_>, input: NoteInputDTO) -> Result<NoteDTO> {
        let db = ctx.data::<model::Database>().unwrap();
        let user = auth::get_user_from_ctx(ctx).await?;
        Ok(NoteDTO::from(
            service::create_note(db, NoteInputActiveModel::from(input).0, user.id).await?,
        ))
    }

    #[graphql(guard = "auth::LoginRequired::new()")]
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: i32,
        update: NoteUpdateDTO,
    ) -> Result<NoteDTO> {
        let db = ctx.data::<model::Database>().unwrap();
        Ok(NoteDTO::from(
            service::update_note(db, id, NoteUpdateActiveModel::from(update).0).await?,
        ))
    }

    #[graphql(guard = "auth::LoginRequired::new()")]
    async fn delete_note(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let db = ctx.data::<model::Database>().unwrap();
        let user = auth::get_user_from_ctx(ctx).await?;
        service::delete_note(db, id, Some(user.id)).await?;
        Ok(true)
    }
}

struct NoteInputActiveModel(model::note::ActiveModel);

impl From<NoteInputDTO> for NoteInputActiveModel {
    fn from(input: NoteInputDTO) -> Self {
        Self {
            0: model::note::ActiveModel {
                title: Set(input.title),
                content: Set(input.content),
                ..Default::default()
            },
        }
    }
}

struct NoteUpdateActiveModel(model::note::ActiveModel);

impl From<NoteUpdateDTO> for NoteUpdateActiveModel {
    fn from(update: NoteUpdateDTO) -> Self {
        Self {
            0: model::note::ActiveModel {
                title: match update.title {
                    None => NotSet,
                    Some(newtitle) => Set(newtitle),
                },
                content: match update.content {
                    None => NotSet,
                    Some(newcontent) => Set(newcontent),
                },
                ..Default::default()
            },
        }
    }
}
