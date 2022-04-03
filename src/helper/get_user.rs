#[macro_export]
macro_rules! get_user {
    ($ctx:ident) => {{
        let _session = $ctx.data::<session::SessionModule>()?;
        let _token = $ctx.data::<crate::session::Session>()?;
        Ok::<model::user::Model, Box<dyn std::error::Error + Send + Sync>>(
            _session.deserialize(_token).await?,
        )
    }};
}
