use dioxus::prelude::*;

/// Display a modal spinner
#[inline_props]
pub fn loading(cx: Scope) -> Element {
    cx.render(rsx! {
        div{
            span{
                // gif here
            }
            p{"loading"}
        }
    })
}
