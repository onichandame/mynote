use async_graphql::{async_trait::async_trait, SimpleObject};
use crud::{Authorizer, Hook, CRUD};
use sea_orm::Set;

use crate::auth::Session;

#[derive(SimpleObject, CRUD)]
#[crud(model = "entity::note", deletable, subscribable)]
pub struct Note {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,

    pub author_id: i32,
    #[crud(creatable, updatable)]
    pub title: String,
    #[crud(creatable, updatable)]
    pub content: String,
}

#[async_trait]
impl Hook for Note {
    type ActiveModel = entity::note::ActiveModel;
    async fn before_create(
        _ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.created_at = Set(chrono::Utc::now().naive_utc());
        Ok(input)
    }
    async fn before_update(
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(input)
    }
}

#[async_trait]
impl Authorizer for Note {
    async fn authorize(
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<sea_orm::Condition> {
        use sea_orm::prelude::*;
        let db = ctx.data::<DatabaseConnection>()?;
        let session = ctx.data::<Session>()?;
        let user = session.decode(db).await?;
        Ok(sea_orm::Condition::all().add(entity::note::Column::AuthorId.eq(user.id)))
    }
}
