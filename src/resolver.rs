use async_graphql::{MergedObject, MergedSubscription};

use crate::{
    auth::{AuthMutation, AuthQuery},
    note::{NoteMutation, NoteQuery, NoteSubscription},
    password::{PasswordMutation, PasswordQuery, PasswordSubscription},
    sync::SyncMutation,
    user::{UserMutation, UserQuery},
};

#[derive(MergedObject, Default)]
pub struct Query(NoteQuery, PasswordQuery, UserQuery, AuthQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(
    NoteMutation,
    PasswordMutation,
    UserMutation,
    AuthMutation,
    SyncMutation,
);

#[derive(MergedSubscription, Default)]
pub struct Subscription(NoteSubscription, PasswordSubscription);
