use chrono::NaiveDateTime;
use filter::Filter;
use merge::Merge;
use sea_orm::Condition;

#[derive(Default, Clone, Merge)]
pub struct PasswordFilter {
    pub id: Option<Filter<i32>>,
    pub user_id: Option<Filter<i32>>,
    pub deleted_at: Option<Filter<NaiveDateTime>>,
}

#[derive(Default, Clone, Merge)]
pub struct PasswordGroupFilter {
    pub id: Option<Filter<i32>>,
    pub user_id: Option<Filter<i32>>,
    pub deleted_at: Option<Filter<NaiveDateTime>>,
}

impl PasswordFilter {
    pub fn build(&self) -> Condition {
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
        filter
    }
}

impl PasswordGroupFilter {
    pub fn build(&self) -> Condition {
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
        filter
    }
}
