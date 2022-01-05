use async_graphql::MergedObject;
use auth::{SessionMutation, UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, SessionMutation);
