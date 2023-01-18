use dioxus::prelude::*;
use dioxus_router::Link;

use crate::app::page::_route;

struct Toggler(bool);

impl Toggler {
    fn toggle(&mut self) {
        self.0 = !self.0;
    }

    fn value(&self) -> bool {
        self.0
    }
}

/// ```text
/// -----  ----
/// brand  menu
/// -----  ----
/// ```
/// Brand contains the brand button and a burger button. Menu contains all the navigations
pub fn navbar(cx: Scope) -> Element {
    use_shared_state_provider(&cx, || Toggler(false));
    cx.render(rsx! {
        nav{
            class: "navbar is-link",
            self::brand {},
            self::menu {},
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
fn brand(cx: Scope) -> Element {
    let menu_state = use_shared_state::<Toggler>(&cx).unwrap();
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
                onclick:move |_|{menu_state.write().toggle()},
                class:if menu_state.read().value(){"navbar-burger is-active"}else{"navbar-burger"},
                span{aria_hidden:"true"}
                span{aria_hidden:"true"}
                span{aria_hidden:"true"}
            }
        },
    })
}

/// Shown in desktop(>=1024px), hidden otherwise. Can be toggled by the burger button
#[inline_props]
fn menu(cx: Scope) -> Element {
    let menu_state = use_shared_state::<Toggler>(&cx).unwrap();
    cx.render(rsx! {
        div{
            id:"navigation",
            class:if menu_state.read().value(){"navbar-menu is-active"}else{"navbar-menu"},
            div{
                class:"navbar-start",
                menu_item{ to:_route::MEMOS, name:"Memo" }
                menu_item{ to:_route::SETTINGS, name:"Settings" }
            }
        }
    })
}

#[inline_props]
fn menu_item(cx: Scope, to: &'static str, name: &'static str) -> Element {
    let menu_state = use_shared_state::<Toggler>(&cx).unwrap();
    cx.render(rsx!(Link {
        to: to,
        onclick:move |_|{menu_state.write().toggle()},
        class: "navbar-item",
        *name
    }))
}
