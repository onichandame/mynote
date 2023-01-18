use dioxus::prelude::*;

pub fn not_found(cx: Scope) -> Element {
    cx.render(rsx! {
        div{
            "Page not found"
        }
    })
}
