use async_graphql::MergedObject;

use self::{
    auth::{AuthMutation, AuthQuery},
    note::{NoteMutation, NoteQuery},
    user::UserMutation,
};

mod auth;
mod note;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(AuthQuery, NoteQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation, UserMutation, NoteMutation);
