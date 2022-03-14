use async_graphql::InputObject;
use chrono::NaiveDateTime;
use filter::Filter;

macro_rules! create_filter {
    ($filter:ident, $data:ident) => {
        #[derive(InputObject)]
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

        impl Into<Filter<$data>> for $filter {
            fn into(self) -> Filter<$data> {
                Filter {
                    eq: self.eq,
                    null: self.null,
                    lt: self.lt,
                    lte: self.lte,
                    gt: self.gt,
                    gte: self.gte,
                    like: self.like,
                    and: self.and.map(|v| v.into_iter().map(|v| v.into()).collect()),
                    or: self.or.map(|v| v.into_iter().map(|v| v.into()).collect()),
                    not: self.not,
                }
            }
        }
    };
}

create_filter!(StringFilter, String);
create_filter!(IntFilter, i64);
create_filter!(DateTimeFilter, NaiveDateTime);
