use async_trait::async_trait;
use sea_orm::EntityTrait;

use crate::{ApplyFilter, ApplyPagination, ApplySorting, Pagination, DB};

#[async_trait]
pub trait List<TEntity: EntityTrait, TFilter: ApplyFilter + Sync, TSorting: ApplySorting + Sync>
where
    Self: DB,
{
    async fn list(
        &self,
        filter: &TFilter,
        pagination: &Pagination,
        sorting: &TSorting,
    ) -> Result<Vec<TEntity::Model>, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = TEntity::find();
        query = pagination.apply_pagination(query);
        query = filter.apply_filter(query);
        query = sorting.apply_sorting(query);
        Ok(query.all(self.db()).await?)
    }
}
