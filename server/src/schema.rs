use async_graphql::Schema as GraphqlSchema;

use crate::resolver::{Mutation, Query, Subscription};

pub type Schema = GraphqlSchema<Query, Mutation, Subscription>;
