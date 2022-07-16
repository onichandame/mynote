use async_graphql::async_trait::async_trait;
use crud::Authorizer;
use sea_orm::{ColumnTrait, DatabaseConnection};

use crate::auth::Session;

pub struct NoteAuthorizer {}

#[async_trait]
impl Authorizer for NoteAuthorizer {
    async fn authorize(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<sea_orm::Condition> {
        let session = ctx.data::<Session>()?;
        let db = ctx.data::<DatabaseConnection>()?;
        let user = session.decode(db).await?;
        Ok(sea_orm::Condition::all().add(entity::note::Column::UserId.eq(user.id)))
    }
}
