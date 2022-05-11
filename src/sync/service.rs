use async_graphql::Result;
use client::{Client, LoginInput, PasswordFilter};
use crud::BooleanFilter;
use futures::StreamExt;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, NotSet,
    PaginatorTrait, QueryFilter, QuerySelect, Set, TransactionTrait,
};

pub async fn sync(
    user_id: i32,
    url: &str,
    identity: &str,
    password: &str,
    db: &DatabaseConnection,
) -> Result<()> {
    let client = Client::new(url);
    let session = client
        .login(LoginInput {
            identity: identity.to_owned(),
            password: password.to_owned(),
        })
        .await?;
    let client = client.set_session(&session);
    let mut stream = client.stream_notes().await?;
    while let Some(remote_note) = stream.next().await {
        let remote_note = remote_note?;
        let should_update = model::note::Entity::find()
            .filter(
                Condition::all()
                    .add(model::note::Column::Uuid.eq(remote_note.uuid.clone()))
                    .add(model::note::Column::UpdatedAt.gte(remote_note.updated_at.clone())),
            )
            .offset(0)
            .limit(1)
            .count(db)
            .await?
            == 0;
        if should_update {
            db.transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    model::note::Entity::delete_many()
                        .filter(model::note::Column::Uuid.eq(remote_note.uuid.clone()))
                        .exec(txn)
                        .await?;
                    let mut active_model = model::note::ActiveModel::from(remote_note);
                    active_model.user_id = Set(user_id.to_owned());
                    active_model.id = NotSet;
                    active_model.insert(txn).await?;
                    Ok(())
                })
            })
            .await?;
        }
    }
    let mut stream = client
        .stream_passwords(Some(PasswordFilter {
            is_local: Some(BooleanFilter {
                eq: Some(false),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .await?;
    while let Some(remote_pw) = stream.next().await {
        let remote_pw = remote_pw?;
        let should_update = model::password::Entity::find()
            .filter(
                Condition::all()
                    .add(model::password::Column::Uuid.eq(remote_pw.uuid.clone()))
                    .add(model::password::Column::UpdatedAt.gte(remote_pw.updated_at.clone())),
            )
            .offset(0)
            .limit(1)
            .count(db)
            .await?
            == 0;
        if should_update {
            db.transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    model::password::Entity::delete_many()
                        .filter(model::password::Column::Uuid.eq(remote_pw.uuid.clone()))
                        .exec(txn)
                        .await?;
                    let mut active_model = model::password::ActiveModel::from(remote_pw);
                    active_model.user_id = Set(user_id.to_owned());
                    active_model.id = NotSet;
                    active_model.insert(txn).await?;
                    Ok(())
                })
            })
            .await?;
        }
    }
    Ok(())
}

pub async fn try_sync_from_peer(peer_id: i32, db: &DatabaseConnection) -> Result<()> {
    let (peer, pw) = model::peer::Entity::find_by_id(peer_id)
        .find_with_related(model::password::Entity)
        .one(db)
        .await?
        .ok_or(format!("peer {} not found", peer_id))?;
    if let Some(pw) = pw {
        if let model::password::Model {
            password,
            url: Some(url),
            ..
        } = &pw
        {
            let identity = pw.email.as_ref().or_else(|| pw.username.as_ref());
            if let Some(identity) = identity {
                sync(peer.user_id, url, identity, password, db).await?;
                Ok(())
            } else {
                Err(format!(
                    "credential of peer {} does not have login idenity set",
                    peer_id
                )
                .into())
            }
        } else {
            Err(format!("credential of peer {} does not have url set", peer_id).into())
        }
    } else {
        Err(format!("credentials for peer {} not found", peer_id).into())
    }
}
