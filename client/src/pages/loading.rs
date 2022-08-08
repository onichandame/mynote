use dioxus::prelude::*;

pub fn loading(cx: Scope) -> Element {
    rsx!(cx,div{
        border_style:"solid",
        "loading"})
}
