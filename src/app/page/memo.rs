use dioxus::prelude::*;

use crate::app::component::layout;

pub fn memo(cx: Scope) -> Element {
    cx.render(rsx! {
        layout::layout{"memo"}
    })
}
