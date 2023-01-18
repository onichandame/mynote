use dioxus::prelude::*;

pub fn init_icons(cx: Scope) -> Element {
    cx.render(rsx! {
          script{ src:"https://code.iconify.design/3/3.0.1/iconify.min.js" }
    })
}

#[inline_props]
pub fn icon<'a>(cx: Scope, id: &'a str) -> Element {
    cx.render(rsx!(span{class:"iconify", "data-icon":*id}))
}
