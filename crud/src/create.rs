use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};

use crate::DB;

#[async_trait]
pub trait Create<TActiveModel: ActiveModelTrait>
where
    Self: DB,
    TActiveModel: sea_orm::ActiveModelBehavior + Send,
    <<TActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model:
        IntoActiveModel<TActiveModel>,
{
    async fn create<TInput: IntoActiveModel<TActiveModel> + Send>(
        &self,
        input: TInput,
    ) -> Result<
        <<TActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        Ok(input.into_active_model().insert(self.db()).await?)
    }
}
