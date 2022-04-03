use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};

use crate::{ApplyFilter, DB};

#[async_trait]
pub trait Update<TActiveModel: ActiveModelTrait, TFilter: ApplyFilter + Sync>
where
    Self: DB,
    TActiveModel: sea_orm::ActiveModelBehavior + Send,
    <<TActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model:
        IntoActiveModel<TActiveModel>,
{
    async fn update<TInput: IntoActiveModel<TActiveModel> + Send>(
        &self,
        filter: &TFilter,
        update: TInput,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut query = TActiveModel::Entity::update_many().set(update.into_active_model());
        query = filter.apply_filter(query);
        Ok(query.exec(self.db()).await.map(|_| ())?)
    }
}
