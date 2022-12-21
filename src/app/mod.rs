use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};

mod component;
mod page;

use page::{_route, home, not_found};

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router{
            link{
                href:"https://cdn.jsdelivr.net/npm/uikit@3.15.18/dist/css/uikit.min.css",
                rel:"stylesheet"
            },
            script{
                src:"https://cdn.jsdelivr.net/npm/uikit@3.15.18/dist/js/uikit.min.js"
            },
            script{
                src:"https://cdn.jsdelivr.net/npm/uikit@3.15.18/dist/js/uikit-icons.min.js"
            },
            Route{ to:_route::HOME, home::home{} }
            Route{ to:_route::NOT_FOUND, not_found::not_found{} }
            Redirect{ from:"", to: _route::NOT_FOUND }
        }
    })
}
