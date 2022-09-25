use dioxus::prelude::*;

use crate::component::layout::end_drawer::EndDrawer;

mod end_drawer;

pub fn Layout<'a>(cx: Scope<'a, LayoutProps<'a>>) -> Element {
    rsx!(cx, Fragment {
        header {
            class: "mui-appbar mui--z1 mui-container-fluid",
            div {
                display: "flex",
                flex: "1",
                flex_wrap:"nowrap",
                align_items: "center",
                justify_content: "space-between",
                class: "mui--appbar-height",
                div {
                    class: "mui--text-title",
                    "{cx.props.title}"
                },
                div {
                    EndDrawer {}
                }
            }
        },
        div {
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct LayoutProps<'a> {
    #[props(default = "Notebook")]
    title: &'a str,
    children: Element<'a>,
}
