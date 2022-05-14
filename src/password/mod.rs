use async_graphql::SimpleObject;
use crud::CRUD;

mod authorizer;
mod hook;

#[derive(SimpleObject, CRUD)]
#[crud(
    model = "model::password",
    authorizer = "authorizer::PasswordAuthorizer{}",
    hook = "hook::PasswordHook{}"
)]
pub struct Password {
    #[crud(column = "Id", filter = "crud::IntFilter")]
    pub id: i32,
    #[crud(column = "Uuid", filter = "crud::StringFilter")]
    pub uuid: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[crud(column = "DeletedAt", filter = "crud::UtcDateTimeFilter", updatable)]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    pub user_id: i32,
    #[crud(
        column = "IsLocal",
        filter = "crud::BooleanFilter",
        creatable,
        updatable
    )]
    pub is_local: bool,
    #[crud(creatable, updatable)]
    pub group_id: Option<i32>,
    #[crud(creatable, updatable)]
    pub title: String,
    #[crud(creatable, updatable)]
    pub icon: Option<String>,
    #[crud(creatable, updatable)]
    pub password: String,
    #[crud(creatable, updatable)]
    pub username: Option<String>,
    #[crud(creatable, updatable)]
    pub url: Option<String>,
}

#[derive(SimpleObject, CRUD)]
#[crud(
    model = "model::password_group",
    authorizer = "authorizer::PasswordGroupAuthorizer{}",
    hook = "hook::PasswordGroupHook{}"
)]
pub struct PasswordGroup {
    #[crud(column = "Id", filter = "crud::IntFilter")]
    pub id: i32,
    #[crud(column = "Uuid", filter = "crud::StringFilter")]
    pub uuid: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[crud(column = "DeletedAt", filter = "crud::UtcDateTimeFilter", updatable)]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    pub user_id: i32,
    #[crud(creatable, updatable)]
    pub parent_id: Option<i32>,
    #[crud(creatable, updatable)]
    pub title: String,
}
