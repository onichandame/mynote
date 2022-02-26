use std::error::Error;

use auth::AuthModule;
use db::new_database_connection;
use note::NoteModule;
use sea_orm::DatabaseConnection;
use user::UserModule;

#[derive(Clone)]
pub struct MyNote {
    pub auth: AuthModule,
    pub db: DatabaseConnection,
    pub user: UserModule,
    pub note: NoteModule,
}

impl MyNote {
    pub async fn create() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let db = new_database_connection().await?;
        Ok(Self {
            db: db.clone(),
            auth: AuthModule::new(db.clone()),
            user: UserModule::new(db.clone()),
            note: NoteModule::new(db.clone()),
        })
    }
}
