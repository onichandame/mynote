#[derive(sqlx::FromRow)]
pub struct Note {
    // base + timestamp + persistent
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,

    pub user_id: i64,
    pub title: String,
    pub content: String,
}
