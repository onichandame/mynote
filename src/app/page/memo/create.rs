use dioxus::prelude::*;
use dioxus_router::use_router;
use sea_orm::{ActiveModelTrait, Set};

use crate::app::{component::error, provider::db::Db};

#[derive(Default)]
struct Input {
    content: String,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),
}

impl Input {
    fn into_active_model(self) -> entity::memo::ActiveModel {
        entity::memo::ActiveModel {
            content: Set(self.content),
            uuid: Set(uuid::Uuid::new_v4().to_string()),
            ..Default::default()
        }
    }
}

pub fn create(cx: Scope) -> Element {
    let is_submitting = use_state(&cx, || false);
    let router = use_router(&cx);
    let err = use_state::<Option<Error>>(&cx, || None);
    let Some(db )= use_context::<Db>(&cx) else{return cx.render(rsx!(error::error{"database not initialized"}))};
    let submit = move |input: Input| {
        let db = db.clone();
        let is_submitting = is_submitting.clone();
        let router = router.clone();
        let err = err.clone();
        async move {
            is_submitting.modify(|_| true);
            if let Err(e) = input.into_active_model().insert(&*db).await {
                err.modify(|_| Some(e.into()));
            } else {
                is_submitting.modify(|_| false);
                router.pop_route();
            }
        }
    };
    let cancel = || router.pop_route();
    cx.render(rsx! {
      form{
        onsubmit:move|event|{
          cx.spawn(submit(Input{content:event.data.values.get("content").map(|v|v.to_owned()).unwrap()}));
        },
        div{
          class:"field",
          div{
            class:"control",
            textarea{
              placeholder:"Content",
              name:"content",
              class:"textarea",
            }
          }
        }
        div{
          class:"field is-grouped is-grouped-right",
          div{
            class:"control",
            button{
              disabled:"{**is_submitting}",
              onclick:move|_|cancel(),
              class:"button is-light",
              "Cancel"
            }
          }
          div{
            class:"control",
            button{
              r#type:"submit",
              disabled:"{**is_submitting}",
              class:"button is-link",
              "Create"
            }
          }
          err.as_ref().map(|e|rsx!(
            p{
              class:"help is-danger",
              e.to_string()
            }
          ))
        }
      }
    })
}
