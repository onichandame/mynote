use dioxus::prelude::*;

use crate::component::layout::Layout;

pub fn WelcomePage(cx: Scope) -> Element {
    rsx!(cx, Layout {
        div {
            class: "mui-col",
            align_items: "center",
            p {
                "Welcome to your private notebook"
            }
            p {
                "login/signup"
            }
        }
    })
}
