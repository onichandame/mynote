use async_graphql::{InputObject, MaybeUndefined, SimpleObject};
use model;

#[derive(SimpleObject)]
#[graphql(name = "User")]
pub struct UserDTO {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: String,
    #[graphql(validator(email))]
    pub email: Option<String>,
    #[graphql(validator(url))]
    pub avatar: Option<String>,
}

#[derive(InputObject)]
#[graphql(name = "CreateUserInput")]
pub struct UserCreateDTO {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(InputObject)]
#[graphql(name = "UpdateUserInput")]
pub struct UserUpdateDTO {
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: MaybeUndefined<String>,
    pub avatar: MaybeUndefined<String>,
}

#[derive(InputObject)]
#[graphql(name = "LoginInput")]
pub struct LoginInputDTO {
    pub name_or_email: String,
    pub password: String,
}

impl From<&model::user::Model> for UserDTO {
    fn from(user: &model::user::Model) -> Self {
        Self {
            created_at: user.created_at.clone(),
            deleted_at: user.deleted_at.clone(),
            email: user.email.clone(),
            avatar: user.avatar.clone(),
            id: user.id.clone(),
            name: user.name.clone(),
            updated_at: user.updated_at.clone(),
        }
    }
}
