use async_trait::async_trait;
use sea_orm::EntityTrait;

use crate::{ApplyFilter, DB};

#[async_trait]
pub trait Delete<TEntity: EntityTrait, TFilter: ApplyFilter + Sync>
where
    Self: DB,
{
    async fn delete(
        &self,
        filter: &TFilter,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut query = TEntity::delete_many();
        query = filter.apply_filter(query);
        Ok(query.exec(self.db()).await.map(|_| ())?)
    }
}
