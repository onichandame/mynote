use async_graphql::InputObject;

#[derive(InputObject)]
pub struct NoteInputDTO {
    pub user_id: i64,
    pub title: String,
    pub content: String,
}

#[derive(InputObject)]
pub struct NoteUpdateDTO {
    pub title: Option<String>,
    pub content: Option<String>,
}
