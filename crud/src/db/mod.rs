use sea_orm::DatabaseConnection;

pub trait DB {
    fn db(&self) -> &DatabaseConnection;
}
