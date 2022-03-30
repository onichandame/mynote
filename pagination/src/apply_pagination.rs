use sea_orm::QuerySelect;

pub trait ApplyPagination {
    fn apply_pagination<T: QuerySelect>(&self, query: T) -> T;
}
