use async_graphql::SimpleObject;
use crud::CRUD;

mod authorizer;
mod hook;

#[derive(SimpleObject, CRUD)]
#[crud(
    model = "model::note",
    authorizer = "authorizer::NoteAuthorizer{}",
    hook = "hook::NoteHook{}"
)]
pub struct Note {
    #[crud(column = "Id", filter = "crud::IntFilter")]
    pub id: i32,
    #[crud(column = "Uuid", filter = "crud::StringFilter")]
    pub uuid: String,
    pub lamport_clock: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[crud(column = "DeletedAt", filter = "crud::UtcDateTimeFilter", updatable)]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    pub user_id: i32,
    #[crud(creatable, updatable)]
    pub title: String,
    #[crud(creatable, updatable)]
    pub content: String,
}
