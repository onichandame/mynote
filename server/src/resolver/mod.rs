use async_graphql::{MergedObject, MergedSubscription};

use self::{auth::AuthMutation, user::UserQuery};

mod auth;
mod guards;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription();
