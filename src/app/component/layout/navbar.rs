use dioxus::prelude::*;
use dioxus_router::Link;

use crate::app::page::_route;

pub fn navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav{
            class:"uk-container uk-container-expand uk-background-primary uk-light",
            "uk-navbar":true,
            div{
                class:"uk-navbar-left",
                ul{
                    class:"uk-navbar-nav",
                    li{
                        Link{
                            to: _route::HOME,
                            class:"uk-text-bold uk-text-lead",
                            "Notebook"
                        }
                    }
                }
            },
            div{
                class:"uk-navbar-right",
                ul{
                    class:"uk-navbar-nav",
                    li{
                        "Ending"
                    }
                }
            }
        }
    })
}
