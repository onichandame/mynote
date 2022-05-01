use async_graphql::{MergedObject, MergedSubscription};

use crate::{
    auth::{AuthMutation, AuthQuery},
    note::{NoteMutation, NoteQuery, NoteSubscription},
    sync::SyncMutation,
    user::{UserMutation, UserQuery},
};

#[derive(MergedObject, Default)]
pub struct Query(NoteQuery, UserQuery, AuthQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(NoteMutation, UserMutation, AuthMutation, SyncMutation);

#[derive(MergedSubscription, Default)]
pub struct Subscription(NoteSubscription);
