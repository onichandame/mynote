use async_graphql::MergedObject;
use auth::{SessionMutation, UserMutation, UserQuery};
use note::{NoteMutation, NoteQuery};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, NoteQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, SessionMutation, NoteMutation);
