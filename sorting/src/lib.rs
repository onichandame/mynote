use sea_orm::{ColumnTrait, QueryOrder};

mod apply_sorting;

pub use apply_sorting::*;

pub struct Sorting<T: ColumnTrait> {
    pub field: T,
    pub direction: SortDirection,
}

pub enum SortDirection {
    ASC,
    DESC,
}

impl<T: ColumnTrait> ApplySorting for Sorting<T> {
    fn apply_sorting<TQuery: QueryOrder>(&self, query: TQuery) -> TQuery {
        match self.direction {
            SortDirection::ASC => query.order_by_asc(self.field),
            SortDirection::DESC => query.order_by_desc(self.field),
        }
    }
}

impl<T: ColumnTrait> ApplySorting for Option<Sorting<T>> {
    fn apply_sorting<TQuery: QueryOrder>(&self, query: TQuery) -> TQuery {
        match self {
            Some(v) => v.apply_sorting(query),
            None => query,
        }
    }
}

impl<T: ColumnTrait> ApplySorting for Vec<Sorting<T>> {
    fn apply_sorting<TQuery: QueryOrder>(&self, query: TQuery) -> TQuery {
        self.into_iter()
            .fold(query, |query, v| v.apply_sorting(query))
    }
}

impl<T: ColumnTrait> ApplySorting for Option<Vec<Sorting<T>>> {
    fn apply_sorting<TQuery: QueryOrder>(&self, query: TQuery) -> TQuery {
        match self {
            Some(v) => v.apply_sorting(query),
            None => query,
        }
    }
}
