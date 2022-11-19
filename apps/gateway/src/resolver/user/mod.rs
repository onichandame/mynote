use async_graphql::{async_trait::async_trait, SimpleObject};
use crud::{Authorizer, Hook, Relation, CRUD};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::entity;

#[derive(SimpleObject, CRUD, Relation)]
#[connection(
    name = "memos",
    target_dto = "super::memo::Memo",
    target_model = "entity::memo",
    from = "id",
    to = "author_id"
)]
#[crud(model = "entity::user")]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[crud(updatable)]
    pub name: String,
    #[crud(updatable)]
    #[graphql(validator(email))]
    pub email: Option<String>,
    #[crud(updatable)]
    #[graphql(validator(url))]
    pub avatar: Option<String>,
}

#[async_trait]
impl Authorizer for User {
    async fn authorize(
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<sea_orm::Condition> {
        use sea_orm::prelude::*;
        let user = ctx.data::<entity::user::Model>()?;
        Ok(sea_orm::Condition::all().add(entity::user::Column::Id.eq(user.id)))
    }
}

#[async_trait]
impl Hook for User {
    type ActiveModel = entity::user::ActiveModel;
    async fn before_delete(
        _ctx: &async_graphql::Context<'_>,
        filter: sea_orm::Condition,
        txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<()> {
        let users = entity::user::Entity::find().filter(filter).all(txn).await?;
        // delete related memos
        entity::memo::Entity::delete_many()
            .filter(entity::memo::Column::AuthorId.is_in(users.iter().map(|v| v.id)))
            .exec(txn)
            .await?;
        Ok(())
    }
}
