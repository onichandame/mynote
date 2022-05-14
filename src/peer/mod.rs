use async_graphql::{ComplexObject, Context, SimpleObject};
use crud::CRUD;
use sea_orm::{DatabaseConnection, EntityTrait};

mod authorizer;
mod hook;

#[derive(SimpleObject, CRUD)]
#[crud(
    model = "model::peer",
    authorizer = "authorizer::PeerAuthorizer{}",
    hook = "hook::PeerHook{}"
)]
#[graphql(complex)]
pub struct Peer {
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
        column = "AutoSync",
        filter = "crud::BooleanFilter",
        creatable,
        updatable
    )]
    pub auto_sync: bool,
    #[crud(creatable, updatable)]
    pub password_id: i32,
    #[crud(creatable, updatable)]
    pub title: String,
    #[crud(creatable, updatable)]
    pub icon: Option<String>,
}

#[ComplexObject]
impl Peer {
    async fn password(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<super::password::Password> {
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(model::password::Entity::find_by_id(self.password_id)
            .one(db)
            .await?
            .map(|v| v.into())
            .ok_or(format!("password {} not found", self.password_id))?)
    }
}
