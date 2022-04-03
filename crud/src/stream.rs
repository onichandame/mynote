use async_trait::async_trait;
use futures::{stream::BoxStream, StreamExt};
use sea_orm::EntityTrait;

use crate::{ApplyFilter, DB};

#[async_trait]
pub trait Stream<TEntity: EntityTrait, TFilter: ApplyFilter + Sync>
where
    Self: DB,
{
    async fn stream(
        &self,
        filter: &TFilter,
    ) -> Result<BoxStream<'_, TEntity::Model>, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = TEntity::find();
        query = filter.apply_filter(query);
        Ok(Box::pin(
            query
                .stream(self.db())
                .await?
                .filter_map(|v| async move { v.ok() }),
        ))
    }
}
