use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct UserDTO {
    pub name: String,
}

impl From<entity::user::Model> for UserDTO {
    fn from(user: entity::user::Model) -> Self {
        Self { name: user.name }
    }
}
