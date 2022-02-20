use async_graphql::{InputObject, MaybeUndefined};
use model;
use sea_orm::{ActiveValue, NotSet, Set};

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
    pub name: String,
    pub password: String,
}

trait IntoActiveValue<T> {
    fn into_active_value(&self) -> T;
}

pub trait IntoActiveModel<T> {
    fn into_active_model(&self) -> T;
}

impl IntoActiveModel<model::user::ActiveModel> for UserCreateDTO {
    fn into_active_model(&self) -> model::user::ActiveModel {
        model::user::ActiveModel {
            name: self.name.into_active_value(),
            password: self.password.into_active_value(),
            email: self.email.into_active_value(),
            avatar: self.avatar.into_active_value(),
            ..Default::default()
        }
    }
}

impl IntoActiveModel<model::user::ActiveModel> for UserUpdateDTO {
    fn into_active_model(&self) -> model::user::ActiveModel {
        model::user::ActiveModel {
            name: self.name.into_active_value(),
            password: self.password.into_active_value(),
            email: self.email.into_active_value(),
            avatar: self.avatar.into_active_value(),
            ..Default::default()
        }
    }
}

impl IntoActiveValue<ActiveValue<String>> for String {
    fn into_active_value(&self) -> ActiveValue<String> {
        Set(self.to_owned())
    }
}

impl IntoActiveValue<ActiveValue<Option<String>>> for MaybeUndefined<String> {
    fn into_active_value(&self) -> ActiveValue<Option<String>> {
        match self {
            MaybeUndefined::Undefined => NotSet,
            MaybeUndefined::Null => Set(None),
            MaybeUndefined::Value(val) => Set(Some(val.to_owned())),
        }
    }
}

impl IntoActiveValue<ActiveValue<Option<String>>> for Option<String> {
    fn into_active_value(&self) -> ActiveValue<Option<String>> {
        match self {
            Some(val) => Set(Some(val.to_owned())),
            None => NotSet,
        }
    }
}

impl IntoActiveValue<ActiveValue<String>> for Option<String> {
    fn into_active_value(&self) -> ActiveValue<String> {
        match self {
            Some(val) => Set(val.to_owned()),
            None => NotSet,
        }
    }
}
