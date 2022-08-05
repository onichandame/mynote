use async_graphql::{MergedObject, MergedSubscription};

use self::{
    auth::AuthMutation,
    note::{NoteMutation, NoteQuery, NoteSubscription},
    user::{UserMutation, UserQuery},
};

mod auth;
mod note;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, NoteQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation, UserMutation, NoteMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(NoteSubscription);
