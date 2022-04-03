use sea_orm::QueryOrder;

pub trait ApplySorting {
    fn apply_sorting<TQuery: QueryOrder>(&self, query: TQuery) -> TQuery;
}
