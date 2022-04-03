use std::error::Error;

use async_graphql::{connection::CursorType, InputObject};
use crud::Pagination;

use crate::cursor::Cursor;

#[derive(InputObject, Default)]
#[graphql(name = "Paging")]
pub struct PagingDTO {
    first: Option<u64>,
    after: Option<String>,
}

impl TryInto<Pagination> for PagingDTO {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_into(self) -> Result<Pagination, Self::Error> {
        let mut offset = None;
        if let Some(after) = self.after {
            offset = Some(Cursor::decode_cursor(&after)?.offset)
        }
        Ok(Pagination {
            offset,
            limit: self.first,
            ..Default::default()
        })
    }
}
