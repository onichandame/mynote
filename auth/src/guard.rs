use async_graphql::extensions;
use db::model;
use serde::Deserialize;
use std::sync::Arc;

use crate::service;

pub type Session = String;

pub struct Auth;

#[derive(Default)]
pub struct LoginRequired;

#[async_trait::async_trait]
impl async_graphql::Guard for LoginRequired {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        ctx.data::<model::User>().map(|_| ())
    }
}

struct AuthExtension;

#[async_trait::async_trait]
impl extensions::Extension for AuthExtension {
    async fn prepare_request(
        &self,
        ctx: &extensions::ExtensionContext<'_>,
        request: async_graphql::Request,
        next: extensions::NextPrepareRequest<'_>,
    ) -> async_graphql::ServerResult<async_graphql::Request> {
        let pool = ctx.data::<db::ConnectionPool>().unwrap();
        let mut request = request;
        if let Ok(session) = ctx.data::<Session>() {
            if let Ok(user) = service::deserialize_session(session, pool).await {
                request = request.data(user);
            }
        }
        next.run(ctx, request).await
    }
}
impl extensions::ExtensionFactory for Auth {
    fn create(&self) -> std::sync::Arc<dyn extensions::Extension> {
        Arc::new(AuthExtension)
    }
}

pub async fn pass_session(v: serde_json::Value) -> async_graphql::Result<async_graphql::Data> {
    #[derive(Deserialize)]
    struct Payload {
        session: Session,
    }
    let mut data = async_graphql::Data::default();
    if let Ok(payload) = serde_json::from_value::<Payload>(v) {
        data.insert(payload.session);
    }
    Ok(data)
}
