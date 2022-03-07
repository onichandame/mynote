use sea_orm::{ColumnTrait, QueryOrder};

pub struct Sorting {
    pub field: String,
    pub direction: SortDirection,
}

pub enum SortDirection {
    ASC,
    DESC,
}

impl Sorting {
    pub fn build<TQuery: QueryOrder, TCol: ColumnTrait>(&self, query: TQuery, col: TCol) -> TQuery {
        match self.direction {
            SortDirection::ASC => query.order_by_asc(col),
            SortDirection::DESC => query.order_by_desc(col),
        }
    }
}
