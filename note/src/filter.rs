use chrono::NaiveDateTime;
use filter::Filter;
use merge::Merge;
use sea_orm::{Condition, QueryFilter};

#[derive(Default, Clone, Merge)]
pub struct NoteFilter {
    pub id: Option<Filter<i32>>,
    pub user_id: Option<Filter<i32>>,
    pub deleted_at: Option<Filter<NaiveDateTime>>,
}

impl NoteFilter {
    pub fn apply_filter<T: QueryFilter>(&self, query: T) -> T {
        let mut filter = Condition::all();
        if let Some(id) = &self.id {
            filter = filter.add(id.build(model::note::Column::Id));
        }
        if let Some(user_id) = &self.user_id {
            filter = filter.add(user_id.build(model::note::Column::UserId));
        }
        if let Some(deleted_at) = &self.deleted_at {
            filter = filter.add(deleted_at.build(model::note::Column::DeletedAt));
        }
        query.filter(filter)
    }
}
