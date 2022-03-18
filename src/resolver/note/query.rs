use async_graphql::{
    connection::{Connection, Edge},
    Context, Object, Result,
};
use filter::Filter;
use merge::Merge;
use note::{NoteFilter, NoteModule};
use pagination::Pagination;

use crate::{
    cursor::Cursor,
    dto::{NoteDTO, NoteFilterDTO, SortingDTO},
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
        first: Option<u64>,
        after: Option<String>,
        filter: Option<NoteFilterDTO>,
        sorting: Option<Vec<SortingDTO>>,
    ) -> Result<Connection<String, NoteDTO>> {
        get_user!(user, ctx);
        let note = ctx.data::<NoteModule>()?;
        let aux_filter = filter;
        let mut filter = NoteFilter {
            user_id: Some(Filter {
                eq: Some(user.id),
                ..Default::default()
            }),
            ..Default::default()
        };
        if let Some(aux) = aux_filter {
            filter.merge(aux.into());
        }
        let pagination = Pagination {
            offset: after.and_then(|v| Cursor::parse(&v).ok().map(|v| v.offset)),
            limit: first,
        };
        let sorting = sorting.map(|v| v.into_iter().map(|v| v.into()).collect());
        let notes = note
            .list(Some(filter.clone()), Some(pagination.clone()), sorting)
            .await?;
        let notes_count = note.count(Some(filter.clone())).await?;
        let mut connection = Connection::new(
            pagination.has_prev(),
            pagination.has_next(notes_count.try_into()?),
        );
        connection.append(notes.iter().enumerate().map(|(ind, val)| {
            Edge::new(
                Cursor {
                    offset: match pagination.offset {
                        Some(offset) => offset,
                        None => 0,
                    } + ind as u64, // may fail for extremely large ind. need optimization
                }
                .to_string(),
                NoteDTO::from(val),
            )
        }));
        Ok(connection)
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn get_note(&self, ctx: &Context<'_>, id: i32) -> Result<NoteDTO> {
        get_user!(user, ctx);
        let note = ctx.data::<NoteModule>()?;
        Ok(NoteDTO::from(
            &note
                .get(NoteFilter {
                    id: Some(Filter {
                        eq: Some(id),
                        ..Default::default()
                    }),
                    user_id: Some(Filter {
                        eq: Some(user.id),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .await?,
        ))
    }
}
