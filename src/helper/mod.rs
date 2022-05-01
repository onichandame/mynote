pub async fn get_user_from_ctx(
    ctx: &async_graphql::Context<'_>,
) -> async_graphql::Result<model::user::Model> {
    let session = ctx.data::<session::SessionModule>()?;
    let token = ctx.data::<crate::session::Session>()?;
    Ok(session.deserialize(token).await?)
}
