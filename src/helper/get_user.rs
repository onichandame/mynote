#[macro_export]
macro_rules! get_user {
    ($x:ident,$ctx:ident) => {
        let _session = $ctx.data::<session::SessionModule>()?;
        let _token = $ctx.data::<crate::session::Session>()?;
        let $x = _session.deserialize(_token).await?;
    };
}
