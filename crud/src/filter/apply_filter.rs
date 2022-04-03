use sea_orm::QueryFilter;

use super::FilterBuilder;

pub trait ApplyFilter {
    fn apply_filter<T: QueryFilter>(&self, query: T) -> T;
}

impl<TFilter: FilterBuilder> ApplyFilter for TFilter {
    fn apply_filter<T: QueryFilter>(&self, query: T) -> T {
        query.filter(self.build())
    }
}

impl<TFilter: ApplyFilter> ApplyFilter for Option<TFilter> {
    fn apply_filter<T: QueryFilter>(&self, query: T) -> T {
        match self {
            Some(v) => v.apply_filter(query),
            None => query,
        }
    }
}
