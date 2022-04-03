use async_graphql::{Context, Result, Subscription};
use crud::{Filter, Stream};
use futures::StreamExt;
use note::NoteModule;

use crate::{dto::NoteDTO, get_user};

#[derive(Default)]
pub struct NoteSubscription {}

#[Subscription]
impl NoteSubscription {
    #[graphql("guard=LoginRequired::new()")]
    async fn stream_notes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<impl futures::Stream<Item = NoteDTO> + 'ctx> {
        let user = get_user!(ctx)?;
        let notes = ctx.data::<NoteModule>()?;
        Ok(notes
            .stream(&note::NoteFilter {
                user_id: Some(Filter {
                    eq: Some(user.id),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await?
            .then(|v| async move { v.into() }))
    }
}
