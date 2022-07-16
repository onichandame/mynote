use async_graphql::SimpleObject;
use crud::CRUD;

mod authorizer;

#[derive(SimpleObject, CRUD)]
#[crud(
    model = "entity::user",
    authorizer = "self::authorizer::UserAuthorizer{}"
)]
pub struct User {
    #[crud(column = "Id", filter = "crud::IntFilter")]
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    #[crud(updatable)]
    pub name: String,
    #[crud(updatable)]
    #[graphql(validator(email))]
    pub email: Option<String>,
    #[crud(updatable)]
    #[graphql(validator(url))]
    pub avatar: Option<String>,
}
