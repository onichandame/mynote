use std::time::Duration;

use async_graphql::{Context, InputObject, Object, Result};
use futures::StreamExt;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::time::sleep;

use crate::helper::get_user_from_ctx;

use self::service::try_sync_from_peer;

mod service;

pub struct SyncDaemon {
    db: DatabaseConnection,
}

#[derive(Default)]
pub struct SyncMutation {}

#[derive(InputObject)]
pub struct SyncFromRemoteInput {
    url: String,
    identity: String,
    password: String,
}

#[Object]
impl SyncMutation {
    async fn sync_from_peer(&self, ctx: &Context<'_>, peer_id: i32) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = get_user_from_ctx(ctx).await?;
        let peer = model::peer::Entity::find()
            .filter(model::peer::Column::Id.eq(peer_id))
            .filter(model::peer::Column::UserId.eq(user.id))
            .filter(model::peer::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(format!("peer {} not found", peer_id))?;
        try_sync_from_peer(peer.id, db).await?;
        Ok(true)
    }
}

impl SyncDaemon {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    /** Starts the daemon and blocks on await indefinitely */
    pub async fn start(self) {
        loop {
            self.sync_once().await.ok();
            #[cfg(debug_assertions)]
            let interval = Duration::from_secs(5);
            #[cfg(not(debug_assertions))]
            let interval = Duration::from_secs(5 * 60);
            sleep(interval).await;
        }
    }

    async fn sync_once(&self) -> Result<()> {
        let mut stream = model::peer::Entity::find()
            .filter(model::peer::Column::DeletedAt.is_null())
            .filter(model::peer::Column::AutoSync.eq(true))
            .stream(&self.db)
            .await?;
        while let Some(Ok(peer)) = stream.next().await {
            service::try_sync_from_peer(peer.id, &self.db).await?;
        }
        Ok(())
    }
}
