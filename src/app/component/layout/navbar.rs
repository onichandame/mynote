use dioxus::prelude::*;
use dioxus_router::Link;

use crate::app::page::_route;

/// ```text
/// -----  ----
/// brand  menu
/// -----  ----
/// ```
/// Brand contains the brand button and a burger button. Menu contains all the navigations
pub fn navbar(cx: Scope) -> Element {
    let menu_state = use_state(&cx, || false);
    cx.render(rsx! {
        nav{
            class: "navbar is-link",
            self::brand {toggle_menu:||menu_state.modify(|prev|!prev), menu_open:**menu_state},
            self::menu {open:**menu_state},
        }
    })
}

/// ```text
/// -----  ----------------
/// brand  burger(<=1024px)
/// -----  ----------------
/// ```
/// The brand button redirects to the homepage. The burger button toggles the menu
#[inline_props]
fn brand<T>(cx: Scope, toggle_menu: T, menu_open: bool) -> Element
where
    T: Fn(),
{
    cx.render(rsx! {
        div{
            class:"navbar-brand",
            Link{
                to: _route::HOME,
                class:"navbar-item",
                h5{
                    style:"text-decoration: none;",
                    "Notebook"
                }
            }
            a{
                role:"button",
                "data-target":"navigation",
                aria_label:"menu",
                aria_expanded:"false",
                onclick:move|_|{toggle_menu()},
                class:if *menu_open{"navbar-burger is-active"}else{"navbar-burger"},
                span{aria_hidden:"true"}
                span{aria_hidden:"true"}
                span{aria_hidden:"true"}
            }
        },
    })
}

/// Shown in desktop(>=1024px), hidden otherwise. Can be toggled by the burger button
#[inline_props]
fn menu(cx: Scope, open: bool) -> Element {
    cx.render(rsx! {
        div{
            id:"navigation",
            class:if *open{"navbar-menu is-active"}else{"navbar-menu"},
            div{
                class:"navbar-start",
                Link{
                    to:_route::MEMO,
                    class:"navbar-item",
                    "Memo"
                }
            }
        }
    })
}
