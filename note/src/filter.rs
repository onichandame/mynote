use chrono::NaiveDateTime;

#[derive(Default, Clone)]
pub struct Filter {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub deleted_at: Option<Option<DateTimeFilter>>,
}

#[derive(Default, Clone)]
pub struct DateTimeFilter {
    pub gt: Option<NaiveDateTime>,
    pub gte: Option<NaiveDateTime>,
    pub lt: Option<NaiveDateTime>,
    pub lte: Option<NaiveDateTime>,
}
