#[derive(sqlx::FromRow)]
pub struct SessionKey {
    // base + timestamp + persistent
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub key: String,
}
