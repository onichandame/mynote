use async_graphql::{MergedObject, MergedSubscription};

use self::{
    auth::AuthMutation,
    memo::{MemoMutation, MemoQuery, MemoSubscription},
    user::{UserMutation, UserQuery},
};

mod auth;
mod memo;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, MemoQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation, UserMutation, MemoMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(MemoSubscription);
