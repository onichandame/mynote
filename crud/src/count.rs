use async_trait::async_trait;
use sea_orm::{EntityTrait, PaginatorTrait};

use crate::{ApplyFilter, DB};

#[async_trait]
pub trait Count<TEntity: EntityTrait, TFilter: ApplyFilter + Sync>
where
    Self: DB,
    TEntity::Model: Sync,
{
    async fn count(
        &self,
        filter: &TFilter,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = TEntity::find();
        query = filter.apply_filter(query);
        Ok(query.count(self.db()).await?)
    }
}
