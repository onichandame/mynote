use async_graphql::{Context, Object, Result};
use filter::Filter;
use note::{NoteFilter, NoteModule};
use session::SessionModule;

use crate::{
    dto::{NoteDTO, NoteInputDTO, NoteUpdateDTO},
    session::Session,
};

#[derive(Default)]
pub struct NoteMutation;

#[Object]
impl NoteMutation {
    #[graphql("guard=LoginRequired::new()")]
    async fn create_note(&self, ctx: &Context<'_>, input: NoteInputDTO) -> Result<NoteDTO> {
        let token = ctx.data::<Session>()?;
        let session = ctx.data::<SessionModule>()?;
        let note = ctx.data::<NoteModule>()?;
        let user = session.deserialize(token).await?;
        Ok(NoteDTO::from(
            &note.create(user.id, &input.title, &input.content).await?,
        ))
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: i32,
        update: NoteUpdateDTO,
    ) -> Result<NoteDTO> {
        let token = ctx.data::<Session>()?;
        let session = ctx.data::<SessionModule>()?;
        let note = ctx.data::<NoteModule>()?;
        let user = session.deserialize(token).await?;
        let filter = NoteFilter {
            user_id: Some(Filter {
                eq: Some(user.id),
                ..Default::default()
            }),
            id: Some(Filter {
                eq: Some(id),
                ..Default::default()
            }),
            ..Default::default()
        };
        note.update(filter.clone(), update.title, update.content)
            .await?;
        Ok(NoteDTO::from(&note.get(filter.clone()).await?))
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn delete_note(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let token = ctx.data::<Session>()?;
        let session = ctx.data::<SessionModule>()?;
        let note = ctx.data::<NoteModule>()?;
        let user = session.deserialize(token).await?;
        note.delete(NoteFilter {
            user_id: Some(Filter {
                eq: Some(user.id),
                ..Default::default()
            }),
            id: Some(Filter {
                eq: Some(id),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;
        Ok(true)
    }
}
