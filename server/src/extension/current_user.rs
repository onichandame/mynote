use std::sync::Arc;

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest},
    Request, ServerResult,
};
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use crate::auth::Session;

use super::utils::error_to_server_error;

pub struct CurrentUser;

impl ExtensionFactory for CurrentUser {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(CurrentUserExtension::default())
    }
}

#[derive(Default)]
struct CurrentUserExtension {}

#[async_trait]
impl Extension for CurrentUserExtension {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        mut request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let db = ctx
            .data::<DatabaseConnection>()
            .map_err(error_to_server_error)?;
        let session = ctx.data::<Session>();
        if let Ok(session) = session {
            request = request.data(session.decode(db).await.map_err(error_to_server_error)?);
        }
        next.run(ctx, request).await
    }
}
