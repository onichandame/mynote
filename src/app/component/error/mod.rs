use dioxus::prelude::*;

/// Display fatal error message when the app cannot function at all
#[inline_props]
pub fn error<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        article {
            class: "message",
            div{
                class: "message-header",
                p {
                    class: "has-text-danger",
                    "Error"
                }
            }
            div{
                class: "message-body",
                children
            }
        }
    })
}
