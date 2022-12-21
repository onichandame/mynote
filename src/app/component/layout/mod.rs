use dioxus::prelude::*;

mod navbar;

#[inline_props]
pub fn layout<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx! {
        header{
            navbar::navbar{}
        }
        main{
            div{
                class:"uk-container",
                children
            }
        }
    })
}
