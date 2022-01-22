use super::{
    dto::{LoginInputDTO, UserInputDTO, UserUpdateDTO},
    get_user_from_ctx,
    guard::LoginRequired,
    service,
};
use async_graphql::{Context, MaybeUndefined, Object, Result};
use dto::UserDTO;
use model;
use sea_orm::{ActiveValue::NotSet, Set};

#[derive(Default)]
pub struct UserQuery;
#[derive(Default)]
pub struct UserMutation;
#[derive(Default)]
pub struct SessionMutation;

#[Object]
impl UserQuery {
    #[graphql(name = "self", guard = "LoginRequired::new()")]
    async fn get_user<'a>(&self, ctx: &Context<'a>) -> Result<UserDTO> {
        Ok(UserDTO::from(get_user_from_ctx(ctx).await?))
    }
}

#[Object]
impl UserMutation {
    async fn register(&self, ctx: &Context<'_>, input: UserInputDTO) -> Result<UserDTO> {
        let db = ctx.data::<model::Database>().unwrap();
        Ok(UserDTO::from(
            service::create_user(db, UserInputActiveModel::from(input).0).await?,
        ))
    }
    #[graphql(guard = "LoginRequired::new()")]
    async fn update_user(&self, ctx: &Context<'_>, update: UserUpdateDTO) -> Result<UserDTO> {
        let db = ctx.data::<model::Database>().unwrap();
        let user = get_user_from_ctx(ctx).await.unwrap();
        Ok(UserDTO::from(
            service::update_user(db, user.id, UserUpdateActiveModel::from(update).0).await?,
        ))
    }
}

#[Object]
impl SessionMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInputDTO) -> Result<String> {
        let db = ctx.data::<model::Database>().unwrap();
        Ok(service::login(db, &input.name, &input.password).await?)
    }
}

struct UserInputActiveModel(pub model::user::ActiveModel);

impl From<UserInputDTO> for UserInputActiveModel {
    fn from(input: UserInputDTO) -> Self {
        Self {
            0: model::user::ActiveModel {
                name: Set(input.name),
                password: Set(input.password),
                avatar: Set(input.avatar),
                email: Set(input.email),
                ..Default::default()
            },
        }
    }
}

struct UserUpdateActiveModel(pub model::user::ActiveModel);

impl From<UserUpdateDTO> for UserUpdateActiveModel {
    fn from(update: UserUpdateDTO) -> Self {
        Self {
            0: model::user::ActiveModel {
                name: match update.name {
                    None => NotSet,
                    Some(newname) => Set(newname),
                },
                password: match update.password {
                    None => NotSet,
                    Some(newpassword) => Set(newpassword),
                },
                email: match update.email {
                    None => NotSet,
                    Some(newemail) => Set(Some(newemail)),
                },
                avatar: match update.avatar {
                    MaybeUndefined::Null => Set(None),
                    MaybeUndefined::Undefined => NotSet,
                    MaybeUndefined::Value(newavatar) => Set(Some(newavatar)),
                },
                ..Default::default()
            },
        }
    }
}
