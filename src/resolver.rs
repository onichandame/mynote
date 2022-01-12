use async_graphql::MergedObject;

use crate::{
    auth::{SessionMutation, UserMutation, UserQuery},
    note::{NoteMutation, NoteQuery},
};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, NoteQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, SessionMutation, NoteMutation);
