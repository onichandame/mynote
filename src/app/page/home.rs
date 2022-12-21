use dioxus::prelude::*;

use crate::app::component::layout;

pub fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        layout::layout{
            "home"
        }
    })
}
