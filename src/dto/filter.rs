use async_graphql::InputObject;
use chrono::NaiveDateTime;
use filter::Filter;

pub trait IntoFilter<T> {
    fn into_filter(&self) -> Filter<T>;
}

macro_rules! create_filter {
    ($filter:ident, $data:ident) => {
        #[derive(InputObject, Clone)]
        pub struct $filter {
            eq: Option<$data>,
            null: Option<bool>,
            lt: Option<$data>,
            lte: Option<$data>,
            gt: Option<$data>,
            gte: Option<$data>,
            like: Option<String>,
            and: Option<Vec<$filter>>,
            or: Option<Vec<$filter>>,
            not: Option<bool>,
        }

        impl IntoFilter<$data> for $filter {
            fn into_filter(&self) -> Filter<$data> {
                Filter {
                    eq: self.eq.clone(),
                    null: self.null,
                    lt: self.lt.clone(),
                    lte: self.lte.clone(),
                    gt: self.gt.clone(),
                    gte: self.gte.clone(),
                    like: self.like.clone(),
                    and: self
                        .and
                        .clone()
                        .map(|v| v.into_iter().map(|v| v.into_filter()).collect()),
                    or: self
                        .or
                        .clone()
                        .map(|v| v.into_iter().map(|v| v.into_filter()).collect()),
                    not: self.not,
                }
            }
        }
    };
}

create_filter!(StringFilter, String);
create_filter!(IntFilter, i64);
create_filter!(DateTimeFilter, NaiveDateTime);
