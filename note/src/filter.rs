use chrono::NaiveDateTime;
use filter::Filter;
use sea_orm::{Condition, QueryFilter};

#[derive(Default, Clone)]
pub struct NoteFilter {
    pub id: Option<Filter<i32>>,
    pub deleted_at: Option<Filter<NaiveDateTime>>,
}

impl NoteFilter {
    pub fn apply_filter<T: QueryFilter>(&self, query: T) -> T {
        let mut filter = Condition::all();
        if let Some(id) = &self.id {
            filter = filter.add(id.build(model::note::Column::Id));
        }
        if let Some(deleted_at) = &self.deleted_at {
            filter = filter.add(deleted_at.build(model::note::Column::DeletedAt));
        }
        query.filter(filter)
    }
}
