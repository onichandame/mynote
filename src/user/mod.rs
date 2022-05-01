use async_graphql::SimpleObject;
use crud::CRUD;

mod authorizer;

#[derive(SimpleObject, CRUD)]
#[crud(model = "model::user", authorizer = "authorizer::UserAuthorizer{}")]
pub struct User {
    #[crud(column = "Id", filter = "crud::IntFilter")]
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    #[crud(creatable, updatable)]
    pub name: String,
    #[crud(creatable, updatable)]
    pub password: String,
    #[crud(creatable, updatable)]
    #[graphql(validator(email))]
    pub email: Option<String>,
    #[crud(creatable, updatable)]
    #[graphql(validator(url))]
    pub avatar: Option<String>,
}
