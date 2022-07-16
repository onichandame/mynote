use async_graphql::async_trait::async_trait;
use crud::Authorizer;
use sea_orm::{ColumnTrait, DatabaseConnection};

use crate::auth::Session;

pub struct UserAuthorizer {}

#[async_trait]
impl Authorizer for UserAuthorizer {
    async fn authorize(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<sea_orm::Condition> {
        let db = ctx.data::<DatabaseConnection>()?;
        let session = ctx.data::<Session>()?;
        let user = session.decode(db).await?;
        Ok(sea_orm::Condition::all().add(entity::user::Column::Id.eq(user.id)))
    }
}
