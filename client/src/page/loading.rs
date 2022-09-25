use dioxus::prelude::*;

use crate::component::layout::Layout;

pub fn LoadingPage<'a>(cx: Scope<'a>) -> Element {
    rsx!(cx, Layout { title: "Loading" })
}
