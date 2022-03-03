use sea_orm::DatabaseConnection;
use session::SessionModule;
use std::error::Error;

use auth::AuthModule;
use db::new_database_connection;
use note::NoteModule;
use user::UserModule;

// re-export structs for DTO
pub use note;

/// Every instance of MyNote is an independent application
#[derive(Clone)]
pub struct MyNote {
    pub auth: AuthModule,
    pub db: DatabaseConnection,
    pub user: UserModule,
    pub note: NoteModule,
    pub session: SessionModule,
}

impl MyNote {
    pub async fn create() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let db = new_database_connection().await?;
        Ok(Self {
            db: db.clone(),
            auth: AuthModule::new(db.clone()),
            user: UserModule::new(db.clone()),
            note: NoteModule::new(db.clone()),
            session: SessionModule::new(db.clone()),
        })
    }
}
