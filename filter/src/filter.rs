use async_graphql::{InputObject, InputType};

#[derive(InputObject, Debug)]
#[graphql(concrete(name="StringFilter", params=(String)))]
pub struct Filter<TData: InputType> {
    gt: Option<TData>,
    gte: Option<TData>,
    lt: Option<TData>,
    lte: Option<TData>,
    eq: Option<TData>,
    is: Option<TData>,
    and: Option<Vec<Filter<TData>>>,
    or: Option<Vec<Filter<TData>>>,
    not: Option<Vec<Filter<TData>>>,
}
