use dioxus::prelude::*;

use crate::app::component::layout;

pub fn not_found(cx: Scope) -> Element {
    cx.render(rsx! {
        layout::layout{
            "Page not found"
        }
    })
}
