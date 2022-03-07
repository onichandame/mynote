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
            like: Option<$data>,
            and: Option<Vec<$filter>>,
            or: Option<Vec<$filter>>,
            not: Option<Box<$filter>>,
        }

        impl Into<Filter<$data>> for $filter {
            fn into(self) -> Filter<$data> {
                Filter {
                    ..Default::default()
                }
            }
        }
    };
}

create_filter!(StringFilter, String);
create_filter!(IntFilter, i64);
create_filter!(DateTimeFilter, NaiveDateTime);
