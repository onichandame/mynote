use async_graphql::{MergedObject, MergedSubscription};

use self::{
    auth::{AuthMutation, AuthQuery},
    note::{NoteMutation, NoteQuery, NoteSubscription},
    user::UserMutation,
};

mod auth;
mod note;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(AuthQuery, NoteQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation, UserMutation, NoteMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(NoteSubscription);
