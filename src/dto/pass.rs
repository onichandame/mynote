use async_graphql::{InputObject, SimpleObject};

#[derive(SimpleObject)]
#[graphql(name = "Pass")]
pub struct PassDTO {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub user_id: i32,
    pub name: String,
    pub password: String,
}

#[derive(InputObject)]
#[graphql(name = "PassInput")]
pub struct PassInputDTO {
    pub name: String,
    pub password: String,
}

impl From<&model::password::Model> for PassDTO {
    fn from(pass: &model::password::Model) -> Self {
        Self {
            created_at: pass.created_at.clone(),
            deleted_at: pass.deleted_at.clone(),
            user_id: pass.user_id.clone(),
            id: pass.id.clone(),
            updated_at: pass.updated_at.clone(),
            name: pass.name.clone(),
            password: pass.password.clone(),
        }
    }
}
