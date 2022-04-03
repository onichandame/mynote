use async_graphql::{
    connection::{Connection, Edge},
    Context, Object, Result,
};
use crud::{Count, Get, List};
use crud::{Filter, Private, Undeleted};
use note::{NoteFilter, NoteModule};

use crate::{
    cursor::Cursor,
    dto::{NoteDTO, NoteFilterDTO, NoteSortingDTO, PagingDTO},
    get_user,
};

#[derive(Default)]
pub struct NoteQuery;

#[Object]
impl NoteQuery {
    #[graphql("guard=LoginRequired::new()")]
    async fn list_notes(
        &self,
        ctx: &Context<'_>,
        paging: Option<PagingDTO>,
        filter: Option<NoteFilterDTO>,
        sorting: Option<Vec<NoteSortingDTO>>,
    ) -> Result<Connection<Cursor, NoteDTO>> {
        let user = get_user!(ctx)?;
        let note = ctx.data::<NoteModule>()?;
        let mut filter: NoteFilter = filter.unwrap_or_default().into();
        filter = filter.private(&user).undeleted();
        let pagination = paging.unwrap_or_default().try_into()?;
        let sorting = sorting
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect();
        let notes = note.list(&filter, &pagination, &sorting).await?;
        let notes_count = note.count(&filter).await?;
        let mut connection = Connection::new(
            pagination.has_prev(),
            pagination.has_next(notes_count.try_into()?),
        );
        connection.append(notes.iter().enumerate().map(|(ind, val)| {
            Edge::new(
                Cursor {
                    offset: pagination.offset.unwrap_or_default() + ind as u64, // may fail for extremely large ind. need optimization
                },
                val.to_owned().into(),
            )
        }));
        Ok(connection)
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn get_note(&self, ctx: &Context<'_>, id: i32) -> Result<NoteDTO> {
        let user = get_user!(ctx)?;
        let note = ctx.data::<NoteModule>()?;
        let mut filter = NoteFilter {
            id: Some(Filter {
                eq: Some(id),
                ..Default::default()
            }),
            ..Default::default()
        };
        filter = filter.private(&user).undeleted();
        Ok(note.get(&filter).await?.into())
    }
}
