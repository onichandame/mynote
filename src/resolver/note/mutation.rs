use async_graphql::{Context, Object, Result};
use crud::{Create, Filter, Get, Private, Update};
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
        let user = get_user!(ctx)?;
        Ok(note
            .create(model::note::Insert {
                user_id: user.id,
                ..input.into()
            })
            .await?
            .into())
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: i32,
        update: NoteUpdateDTO,
    ) -> Result<NoteDTO> {
        let user = get_user!(ctx)?;
        let note = ctx.data::<NoteModule>()?;
        let filter = NoteFilter {
            id: Some(Filter {
                eq: Some(id),
                ..Default::default()
            }),
            ..Default::default()
        }
        .private(&user);
        note.update(&filter, Into::<model::note::Update>::into(update))
            .await?;
        Ok(note.get(&filter).await?.into())
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn delete_note(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let user = get_user!(ctx)?;
        let note = ctx.data::<NoteModule>()?;
        note.update(
            &NoteFilter {
                id: Some(Filter {
                    eq: Some(id),
                    ..Default::default()
                }),
                ..Default::default()
            }
            .private(&user),
            model::note::Update {
                deleted_at: Some(Some(chrono::Utc::now().naive_utc())),
                ..Default::default()
            },
        )
        .await?;
        Ok(true)
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn sync_notes(
        &self,
        ctx: &Context<'_>,
        remote_username: String,
        remote_password: String,
        url: String,
    ) -> Result<bool> {
        let user = get_user!(ctx)?;
        let note = ctx.data::<NoteModule>()?;
        note.sync_from(&url, &remote_username, &remote_password, user.id)
            .await?;
        Ok(true)
    }
}
