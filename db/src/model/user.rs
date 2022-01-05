use sqlx;

#[derive(sqlx::FromRow)]
pub struct User {
    // base + timestamp + persistent
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub name: String,
    pub password: String,
    pub email: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            created_at: chrono::offset::Utc::now().naive_utc(),
            deleted_at: None,
            updated_at: None,
            email: None,
            name: "".to_string(),
            password: "".to_string(),
        }
    }
}
