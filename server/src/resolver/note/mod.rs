use async_graphql::SimpleObject;
use crud::CRUD;

mod authorizer;
mod hook;

#[derive(SimpleObject, CRUD)]
#[crud(
    model = "entity::note",
    authorizer = "authorizer::NoteAuthorizer{}",
    hook = "hook::NoteHook{}",
    deletable
)]
pub struct Note {
    #[crud(column = "Id", filter = "crud::IntFilter")]
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[crud(column = "DeletedAt", filter = "crud::UtcDateTimeFilter", updatable)]
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub user_id: i32,
    #[crud(creatable, updatable)]
    pub title: String,
    #[crud(creatable, updatable)]
    pub content: String,
}
