use sea_orm::{DatabaseConnection, SqlxSqliteConnector};

pub use note::Entity as Note;
pub use session_key::Entity as SessionKey;
pub use user::Entity as User;

pub mod note;
pub mod session_key;
pub mod user;

pub type Database = DatabaseConnection;

pub async fn new_connection() -> Database {
    SqlxSqliteConnector::from_sqlx_sqlite_pool(db::new_connection_pool().await)
}
