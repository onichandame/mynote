use dioxus::prelude::*;

pub fn EndDrawer(cx: Scope) -> Element {
    rsx!(
        cx,
        Fragment{
            span {
                font_size: "2rem",
                class: "material-icons",
                "account_circle"
            },
            div {
                "drawer"
            }
        }
    )
}
