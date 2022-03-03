use async_graphql::InputObject;
use chrono::NaiveDateTime;

#[derive(InputObject)]
pub struct DateTimeFilter {
    gt: Option<NaiveDateTime>,
    gte: Option<NaiveDateTime>,
    lt: Option<NaiveDateTime>,
    lte: Option<NaiveDateTime>,
}
