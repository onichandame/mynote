use async_graphql::{Context, Object, Result};
use auth;
use db::model;
use sqlx;

use crate::dto::NoteUpdateDTO;
use crate::dto::{NoteDTO, NoteInputDTO};

#[derive(Default)]
pub struct NoteQuery;
#[derive(Default)]
pub struct NoteMutation;

#[Object]
impl NoteQuery {
    #[graphql(guard = "auth::LoginRequired::new()")]
    async fn list_notes<'a>(&self, ctx: &Context<'a>) -> Result<Vec<NoteDTO>> {
        let pool = ctx.data::<db::ConnectionPool>().unwrap();
        let user = auth::get_user_from_ctx(ctx).await?;
        Ok(
            sqlx::query_as::<_, model::Note>("SELECT * FROM notes WHERE user_id = ?")
                .bind(user.id)
                .fetch_all(pool)
                .await?
                .iter()
                .map(|note| NoteDTO::from(note))
                .collect::<Vec<_>>(),
        )
    }
}

#[Object]
impl NoteMutation {
    #[graphql(guard = "auth::LoginRequired::new()")]
    async fn create_note(&self, ctx: &Context<'_>, input: NoteInputDTO) -> Result<NoteDTO> {
        let pool = ctx.data::<db::ConnectionPool>().unwrap();
        Ok(NoteDTO::from(
            sqlx::query_as::<_, model::Note>(
                "INSERT INTO notes (user_id, title, content) VALUES (?,?,?) RETURNING *",
            )
            .bind(input.user_id)
            .bind(input.title)
            .bind(input.content)
            .fetch_one(pool)
            .await?,
        ))
    }
    #[graphql(guard = "auth::LoginRequired::new()")]
    async fn update_note(
        &self,
        ctx: &Context<'_>,
        id: i64,
        update: NoteUpdateDTO,
    ) -> Result<NoteDTO> {
        let pool = ctx.data::<db::ConnectionPool>().unwrap();
        let user = auth::get_user_from_ctx(ctx).await?;
        let mut update_query = vec!["updated_at=now"];
        if let Some(_) = &update.title {
            update_query.push("title=?")
        }
        if let Some(_) = &update.content {
            update_query.push("content=?")
        }
        let query_str = vec![
            "UPDATE notes SET",
            &update_query.join(","),
            "WHERE id=? AND user_id=?",
        ]
        .join(" ");
        let mut query = sqlx::query_as::<sqlx::Sqlite, model::Note>(&query_str);
        if let Some(title) = &update.title {
            query = query.bind(title)
        }
        if let Some(content) = &update.content {
            query = query.bind(content)
        }
        query = query.bind(id);
        query = query.bind(user.id);
        Ok(NoteDTO::from(query.fetch_one(pool).await?))
    }
}
