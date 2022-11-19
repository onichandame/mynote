use async_graphql::{MergedObject, MergedSubscription};

use self::{
    auth::AuthMutation,
    content::ContentQuery,
    memo::{MemoMutation, MemoQuery, MemoSubscription},
    user::{UserMutation, UserQuery},
};

mod auth;
mod content;
mod memo;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, MemoQuery, ContentQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation, UserMutation, MemoMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(MemoSubscription);
