use async_graphql::{
    connection::{Connection, Edge},
    Context, MergedObject, Object, Result,
};
use auth::AuthModule;
use session::SessionModule;
use user::UserModule;

use crate::{
    conversion::IntoUniversal,
    dto::{
        LoginInputDTO, NoteDTO, NoteFilterDTO, NoteInputDTO, NoteUpdateDTO, UserCreateDTO, UserDTO,
        UserUpdateDTO,
    },
    guard::LoginRequired,
    session::Session,
};

#[derive(MergedObject, Default)]
pub struct Query(AuthQuery, NoteQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(AuthMutation, UserMutation, NoteMutation);

#[derive(Default)]
struct AuthQuery;
#[derive(Default)]
struct AuthMutation;
#[derive(Default)]
struct UserMutation;
#[derive(Default)]
struct NoteQuery;
#[derive(Default)]
struct NoteMutation;

#[Object]
impl AuthQuery {
    #[graphql(name = "self", guard = "LoginRequired::new()")]
    async fn me(&self, ctx: &Context<'_>) -> Result<UserDTO> {
        let token = ctx.data::<Session>()?;
        let session = ctx.data::<SessionModule>()?;
        Ok(UserDTO::from(&session.deserialize(token).await?))
    }
}

#[Object]
impl AuthMutation {
    async fn sign_up(&self, ctx: &Context<'_>, input: UserCreateDTO) -> Result<UserDTO> {
        let auth = ctx.data::<AuthModule>()?;
        Ok(UserDTO::from(
            &auth
                .signup(&input.name, &input.password, input.email, input.avatar)
                .await?,
        ))
    }
    async fn login(&self, ctx: &Context<'_>, input: LoginInputDTO) -> Result<String> {
        let session = ctx.data::<SessionModule>()?;
        let auth = ctx.data::<AuthModule>()?;
        Ok(session
            .serialize(
                &auth
                    .login_by_password(&input.name_or_email, &input.password)
                    .await?,
            )
            .await?)
    }
}

#[Object]
impl UserMutation {
    async fn update_user(&self, ctx: &Context<'_>, update: UserUpdateDTO) -> Result<UserDTO> {
        let token = ctx.data::<Session>()?;
        let session = ctx.data::<SessionModule>()?;
        let user_module = ctx.data::<UserModule>()?;
        let user = session.deserialize(token).await?;
        Ok(UserDTO::from(
            &user_module
                .update(
                    user.id,
                    update.name,
                    update.password,
                    update.email.into_universal(),
                    update.avatar.into_universal(),
                )
                .await?,
        ))
    }
}

#[Object]
impl NoteQuery {
    #[graphql("guard=LoginRequired::new()")]
    async fn list_notes(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        filter: Option<NoteFilterDTO>,
    ) -> Result<Connection<String, NoteDTO>> {
        let core = ctx.data::<MyNote>()?;
        let session = ctx.data::<Session>()?;
        let user = core.session.deserialize(session).await?;
        let notes = core
            .note
            .list(
                first,
                after,
                Some(note::Filter {
                    user_id: Some(user.id),
                    ..Default::default()
                }),
            )
            .await?;
        let mut connection = Connection::new();
        connection.append(
            notes
                .iter()
                .enumerate()
                .map(|(ind, val)| Edge::new(format!("offset:{}", ind), NoteDTO::from(val))),
        );
        Ok(connection)
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn get_note(&self, ctx: &Context<'_>, id: i32) -> Result<NoteDTO> {
        let core = ctx.data::<MyNote>()?;
        let session = ctx.data::<Session>()?;
        let user = core.auth.get_user_for_session(session).await?;
        let note = core
            .note
            .get(note::Filter {
                id: Some(id),
                user_id: Some(user.id),
                ..Default::default()
            })
            .await?;
        Ok(NoteDTO::from(&note))
    }
}
#[Object]
impl NoteMutation {
    #[graphql("guard=LoginRequired::new()")]
    async fn create_note(&self, ctx: &Context<'_>, input: NoteInputDTO) -> Result<NoteDTO> {
        let core = ctx.data::<MyNote>()?;
        let session = ctx.data::<Session>()?;
        Ok(NoteDTO::from(
            &core
                .note
                .create(
                    core.auth.get_user_for_session(session).await?.id,
                    &input.title,
                    &input.content,
                )
                .await?,
        ))
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: i32,
        update: NoteUpdateDTO,
    ) -> Result<NoteDTO> {
        let core = ctx.data::<MyNote>()?;
        let session = ctx.data::<Session>()?;
        let user = core.auth.get_user_for_session(session).await?;
        let filter = note::Filter {
            user_id: Some(user.id),
            id: Some(id),
            ..Default::default()
        };
        core.note
            .update(filter.clone(), update.title, update.content)
            .await?;
        Ok(NoteDTO::from(&core.note.get(filter.clone()).await?))
    }
    #[graphql("guard=LoginRequired::new()")]
    async fn delete_note(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let core = ctx.data::<MyNote>()?;
        let session = ctx.data::<Session>()?;
        let user = core.auth.get_user_for_session(session).await?;
        core.note
            .delete(note::Filter {
                user_id: Some(user.id),
                id: Some(id),
                ..Default::default()
            })
            .await?;
        Ok(true)
    }
}
