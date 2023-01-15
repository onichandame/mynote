use dioxus::prelude::*;

pub mod db;
pub mod path;

#[inline_props]
pub fn provider<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        path::path_provider{
            db::db_provider{
                children
            }
        }
    })
}
