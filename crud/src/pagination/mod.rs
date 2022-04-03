use sea_orm::QuerySelect;

mod apply_pagination;

pub use apply_pagination::*;

#[derive(Default, Clone)]
pub struct Pagination {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl ApplyPagination for Pagination {
    fn apply_pagination<T: QuerySelect>(&self, query: T) -> T {
        let query = match self.limit {
            Some(v) => query.limit(v),
            None => query,
        };
        let query = match self.offset {
            Some(v) => query.offset(v),
            None => query,
        };
        query
    }
}

impl ApplyPagination for Option<Pagination> {
    fn apply_pagination<T: QuerySelect>(&self, query: T) -> T {
        match self {
            Some(v) => v.apply_pagination(query),
            None => query,
        }
    }
}

impl Pagination {
    pub fn has_prev(&self) -> bool {
        match self.offset {
            Some(offset) => offset > 0,
            None => false,
        }
    }

    pub fn has_next(&self, total_count: u64) -> bool {
        match self.offset {
            Some(offset) => match self.limit {
                Some(limit) => offset + limit < total_count,
                None => false,
            },
            None => false,
        }
    }
}

// TODO: test like filter
