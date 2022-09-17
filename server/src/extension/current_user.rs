use std::sync::Arc;

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest},
    Request, ServerResult,
};
use async_trait::async_trait;

use crate::auth::Session;

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
        let session = ctx.data::<Session>();
        if let Ok(session) = session {
            request = request.data(session.user.clone());
        }
        next.run(ctx, request).await
    }
}
