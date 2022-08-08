use dioxus::prelude::*;

mod navbar;

#[inline_props]
pub fn layout<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    rsx!(
        cx,
        div{
            navbar::navbar{}
            children
        }
    )
}
