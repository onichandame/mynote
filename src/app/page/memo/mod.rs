use dioxus::prelude::*;
use dioxus_router::Link;
use entity::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::app::{
    component::{error, icon, loading},
    page::_route,
    provider::db::Db,
};

mod create;
mod detail;

pub use {create::create, detail::detail};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sea_orm::error::DbErr),
}

pub fn memo(cx: Scope) -> Element {
    let Some(db )= use_context::<Db>(&cx) else {return cx.render(rsx!(error::error{"database not initialized"}))};
    let fut = use_future(&cx, (db,), |(db,)| async move {
        let memos = Memo::find()
            .filter(entity::memo::Column::DeletedAt.is_null())
            .order_by_asc(entity::memo::Column::Weight)
            .order_by_desc(entity::memo::Column::CreatedAt)
            .all(&*db)
            .await?;
        Ok::<Vec<entity::memo::Model>, Error>(memos)
    });
    cx.render(match fut.value() {
        None => rsx!(loading::loading{"loading memos..."}),
        Some(Ok(memos)) => rsx!(div{
            class:"menu",
            ul{
                class:"menu-list",
                memos.into_iter().map(|memo|rsx!(li{
                    memo.content.as_str()
                }))
                li{
                    class:"is-size-5",
                    Link{
                        to:_route::CREATE_MEMO,
                        class:"is-flex is-justify-content-center",
                        div{
                            class:"is-flex",
                            icon::icon{ id: "ic:round-add-circle-outline" }
                        }
                    }
                }
            }
        }),
        Some(Err(e)) => rsx!(error::error{e.to_string()}),
    })
}
