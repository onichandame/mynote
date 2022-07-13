use async_graphql::{MergedObject, MergedSubscription};

mod auth;
mod guards;

#[derive(MergedObject, Default)]
pub struct Query();

#[derive(MergedObject, Default)]
pub struct Mutation();

#[derive(MergedSubscription, Default)]
pub struct Subscription();
