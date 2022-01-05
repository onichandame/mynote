use async_graphql::{InputObject, SimpleObject};
use db::model;

#[derive(SimpleObject)]
#[graphql(name = "User")]
pub struct UserDTO {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub name: String,
    pub email: Option<String>,
}

#[derive(InputObject)]
pub struct UserInputDTO {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(InputObject)]
pub struct LoginInputDTO {
    pub name: String,
    pub password: String,
}

impl From<model::User> for UserDTO {
    fn from(user: model::User) -> Self {
        Self {
            created_at: user.created_at,
            deleted_at: user.deleted_at,
            email: user.email,
            id: user.id,
            name: user.name,
            updated_at: user.updated_at,
        }
    }
}
impl From<&model::User> for UserDTO {
    fn from(user: &model::User) -> Self {
        Self {
            created_at: user.created_at.clone(),
            deleted_at: user.deleted_at.clone(),
            email: user.email.clone(),
            id: user.id.clone(),
            name: user.name.clone(),
            updated_at: user.updated_at.clone(),
        }
    }
}

impl From<model::User> for UserInputDTO {
    fn from(user: model::User) -> Self {
        Self {
            email: user.email,
            name: user.name,
            password: user.password,
        }
    }
}
