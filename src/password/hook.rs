use futures::StreamExt;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};

use crate::helper::get_user_from_ctx;

pub struct PasswordHook {}

pub struct PasswordGroupHook {}

#[async_trait::async_trait]
impl crud::Hook for PasswordHook {
    type ActiveModel = model::password::ActiveModel;
    async fn before_create(
        &self,
        ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        let user = get_user_from_ctx(ctx).await?;
        input.user_id = sea_orm::Set(user.id);
        input.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
        Ok(input)
    }
    async fn before_update(
        &self,
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        mut input: Self::ActiveModel,
        txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
        let soft_deleting = input.deleted_at.clone().into_value().is_some();
        if soft_deleting {
            let referenced_by_peer = model::peer::Entity::find()
                .filter(model::peer::Column::DeletedAt.is_null())
                .offset(0)
                .limit(1)
                .count(txn)
                .await?
                > 0;
            if referenced_by_peer {
                return Err("cannot soft delete password as it is referenced by peers".into());
            }
        }
        Ok(input)
    }
}

#[async_trait::async_trait]
impl crud::Hook for PasswordGroupHook {
    type ActiveModel = model::password_group::ActiveModel;
    async fn before_create(
        &self,
        ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        let user = get_user_from_ctx(ctx).await?;
        input.user_id = sea_orm::Set(user.id);
        input.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
        Ok(input)
    }
    async fn before_update(
        &self,
        _ctx: &async_graphql::Context<'_>,
        filter: sea_orm::Condition,
        mut input: Self::ActiveModel,
        txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        if let ActiveValue::Set(Some(parent_id)) = &input.parent_id {
            let parent = model::password_group::Entity::find_by_id(*parent_id)
                .one(txn)
                .await?
                .ok_or("failed to find parent in update")?;
            let has_loop = model::password_group::Entity::find()
                .filter(filter)
                .stream(txn)
                .await?
                .any(|v| async {
                    match v {
                        Ok(v) => parent.has_ancestor(v.id, txn).await.map_or(false, |v| v),
                        Err(_) => false,
                    }
                })
                .await;
            if has_loop {
                return Err("update cannot form a loop in password groups".into());
            }
        }
        input.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
        Ok(input)
    }
}
