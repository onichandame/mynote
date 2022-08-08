use dioxus::prelude::*;

pub fn navbar(cx: Scope) -> Element {
    rsx!(
        cx,
        div {
            class: "mui-appbar",
            div{
                class: "mui-container-fluid",
                div{
                    padding: "1",
                    align_items: "center",
                    class: "mui-row",
                    div{
                        class: "mui-col",
                        div{
                            class: "mui--text-display1",
                            "notebook"
                        }
                    }
                }
            }
        }
    )
}
