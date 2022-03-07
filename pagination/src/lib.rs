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
}

// TODO: test like filter
