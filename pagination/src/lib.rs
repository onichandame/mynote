use sea_orm::QuerySelect;

#[derive(Default, Clone)]
pub struct Pagination {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl Pagination {
    pub fn build<T: QuerySelect>(&self, mut query: T) -> T {
        if let Some(limit) = &self.limit {
            query = query.limit(*limit);
        }
        if let Some(offset) = &self.offset {
            query = query.offset(*offset);
        }
        query
    }

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
