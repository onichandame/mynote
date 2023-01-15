use dioxus::prelude::*;

pub mod db;
pub mod path;

#[inline_props]
pub fn provider<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        //db::db_provider{
        //    children
        //}
        path::path_provider{
            children
        }
    })
}
