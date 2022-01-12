use async_graphql::{Context, MaybeUndefined, Object, Result};
use db::{model, ConnectionPool};

use crate::{
    dto::{LoginInputDTO, UserDTO, UserInputDTO, UserUpdateDTO},
    guard::LoginRequired,
    service::{self, get_user_from_ctx},
};

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
        let pool = ctx.data::<ConnectionPool>().unwrap();
        Ok(UserDTO::from(
            sqlx::query_as::<_, model::User>(
                "INSERT INTO users (name,email,password) VALUES (?,?,?) RETURNING *",
            )
            .bind(input.name)
            .bind(input.email)
            .bind(bcrypt::hash(input.password, bcrypt::DEFAULT_COST)?)
            .fetch_one(pool)
            .await?,
        ))
    }
    #[graphql(guard = "LoginRequired::new()")]
    async fn update_user(&self, ctx: &Context<'_>, update: UserUpdateDTO) -> Result<UserDTO> {
        let pool = ctx.data::<db::ConnectionPool>().unwrap();
        let user = get_user_from_ctx(ctx).await.unwrap();
        let mut update_str = vec!["updated_at=now"];
        if let Some(_) = update.name {
            update_str.push("name=?");
        }
        if let Some(_) = update.password {
            update_str.push("password=?");
        }
        if let Some(_) = update.email {
            update_str.push("email=?");
        }
        match &update.avatar {
            MaybeUndefined::Value(_) | MaybeUndefined::Null => {
                update_str.push("avatar=?");
            }
            _other => {}
        }
        let query_str = vec![
            "UPDATE users SET",
            &update_str.join(","),
            "WHERE id=? RETURNING *",
        ]
        .join(" ");
        let mut query = sqlx::query_as::<sqlx::Sqlite, model::User>(&query_str);
        if let Some(name) = &update.name {
            query = query.bind(name);
        }
        if let Some(password) = &update.password {
            query = query.bind(bcrypt::hash(password, bcrypt::DEFAULT_COST)?);
        }
        if let Some(email) = &update.email {
            query = query.bind(email);
        }
        match &update.avatar {
            MaybeUndefined::Value(avatar) => {
                query = query.bind(avatar);
            }
            MaybeUndefined::Null => {
                query = query.bind("NULL");
            }
            _other => {}
        }
        query = query.bind(user.id);
        Ok(UserDTO::from(query.fetch_one(pool).await?))
    }
}

#[Object]
impl SessionMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInputDTO) -> Result<String> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        let session = service::login(&input.name, &input.password, pool).await?;
        Ok(session)
    }
}
