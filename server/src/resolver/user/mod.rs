use async_graphql::{async_trait::async_trait, SimpleObject};
use crud::{Authorizer, Hook, CRUD};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::auth::Session;

#[derive(SimpleObject, CRUD)]
#[crud(model = "entity::user")]
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
        let db = ctx.data::<DatabaseConnection>()?;
        let session = ctx.data::<Session>()?;
        let user = session.decode(db).await?;
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
        // delete related notes
        entity::note::Entity::delete_many()
            .filter(entity::note::Column::AuthorId.is_in(users.iter().map(|v| v.id)))
            .exec(txn)
            .await?;
        Ok(())
    }
}
