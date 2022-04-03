use chrono::NaiveDateTime;
use crud::{Filter, FilterBuilder, Private, Undeleted};
use sea_orm::Condition;

#[derive(Default, Clone)]
pub struct NoteFilter {
    pub id: Option<Filter<i32>>,
    pub user_id: Option<Filter<i32>>,
    pub deleted_at: Option<Filter<NaiveDateTime>>,
}

impl FilterBuilder for NoteFilter {
    fn build(&self) -> Condition {
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

impl Private for NoteFilter {
    type User = model::user::Model;
    fn private(self, user: &Self::User) -> Self {
        Self {
            user_id: Some(Filter {
                eq: Some(user.id),
                ..Default::default()
            }),
            ..self
        }
    }
}

impl Undeleted for NoteFilter {
    fn undeleted(self) -> Self {
        Self {
            deleted_at: Some(Filter {
                ..Default::default()
            }),
            ..self
        }
    }
}
