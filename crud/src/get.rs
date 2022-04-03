use async_trait::async_trait;
use sea_orm::EntityTrait;

use crate::{ApplyFilter, DB};

#[async_trait]
pub trait Get<TEntity: EntityTrait, TFilter: ApplyFilter + Sync>
where
    Self: DB,
{
    async fn get(
        &self,
        filter: &TFilter,
    ) -> Result<TEntity::Model, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = TEntity::find();
        query = filter.apply_filter(query);
        Ok(query
            .one(self.db())
            .await?
            .ok_or(format!("note not found",))?)
    }
}
