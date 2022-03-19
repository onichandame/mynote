use async_graphql::{Context, Object, Result};
use filter::Filter;
use note::{NoteFilter, NoteModule};

use crate::{
    dto::{NoteDTO, NoteInputDTO, NoteUpdateDTO},
    get_user,
};

#[derive(Default)]
pub struct NoteMutation;

#[Object]
impl NoteMutation {
    #[graphql("guard=LoginRequired::new()")]
    async fn create_note(&self, ctx: &Context<'_>, input: NoteInputDTO) -> Result<NoteDTO> {
        let note = ctx.data::<NoteModule>()?;
        get_user!(user, ctx);
        Ok(NoteDTO::from(
            &note
                .create(user.id, &input.title, &input.content, None, None)
                .await?,
        ))
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: i32,
        update: NoteUpdateDTO,
    ) -> Result<NoteDTO> {
        get_user!(user, ctx);
        let note = ctx.data::<NoteModule>()?;
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
        note.update(filter.clone(), update.title, update.content, None)
            .await?;
        Ok(NoteDTO::from(&note.get(filter.clone()).await?))
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn delete_note(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        get_user!(user, ctx);
        let note = ctx.data::<NoteModule>()?;
        note.update(
            NoteFilter {
                user_id: Some(Filter {
                    eq: Some(user.id),
                    ..Default::default()
                }),
                id: Some(Filter {
                    eq: Some(id),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
            None,
            Some(Some(chrono::Utc::now().naive_utc())),
        )
        .await?;
        Ok(true)
    }
}
