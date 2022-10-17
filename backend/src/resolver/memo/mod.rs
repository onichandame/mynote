use async_graphql::{async_trait::async_trait, SimpleObject};
use crud::{Authorizer, Hook, Relation, CRUD};
use sea_orm::Set;

use crate::entity;

#[derive(SimpleObject, CRUD, Relation)]
#[relation(
    name = "author",
    target_dto = "super::user::User",
    target_model = "entity::user",
    from = "author_id",
    to = "id"
)]
#[crud(model = "entity::memo", deletable, subscribable)]
#[graphql(complex)]
pub struct Memo {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,

    pub author_id: i32,
    #[crud(creatable, updatable)]
    pub content: String,
    #[crud(creatable, updatable)]
    pub weight: Option<i32>,
}

#[async_trait]
impl Hook for Memo {
    type ActiveModel = entity::memo::ActiveModel;
    async fn before_create(
        ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        let user = ctx.data::<entity::user::Model>()?;
        input.created_at = Set(chrono::Utc::now().naive_utc());
        input.author_id = Set(user.id);
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
impl Authorizer for Memo {
    async fn authorize(
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<sea_orm::Condition> {
        use sea_orm::prelude::*;
        let user = ctx.data::<entity::user::Model>()?;
        Ok(sea_orm::Condition::all().add(entity::memo::Column::AuthorId.eq(user.id)))
    }
}
