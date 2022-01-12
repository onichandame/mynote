use async_graphql::{InputObject, MaybeUndefined};

#[derive(InputObject)]
pub struct UserInputDTO {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(InputObject)]
pub struct UserUpdateDTO {
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub avatar: MaybeUndefined<String>,
}

#[derive(InputObject)]
pub struct LoginInputDTO {
    pub name: String,
    pub password: String,
}
