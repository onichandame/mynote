use dioxus::prelude::*;

/// Display a modal spinner
#[inline_props]
pub fn loading<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div{
            class:"modal is-active",
            div{class:"modal-background"}
            div{
                class:"modal-content is-flex is-justify-content-center no-scrollbar",
                div{
                    class:"is-flex is-flex-direction-column is-align-items-center",
                    div{
                        class:"is-flex",
                        span{class:"spinner"}
                    }
                    div{
                        class:"is-flex is-size-4 has-text-light",
                        children
                    }
                }
            }
        }
    })
}
