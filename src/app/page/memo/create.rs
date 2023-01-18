use dioxus::prelude::*;

pub fn create(cx: Scope) -> Element {
    cx.render(rsx! {
      div{"create"}
    })
}
