use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};

mod component;
mod page;
mod provider;

use crate::app::page::{_route, home, memo, not_found};

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel:"stylesheet", href:"https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css" }
        style{
            vec![include_str!("app.css")].into_iter().map(|v|rsx!(v))
        }
        provider::provider{
            Router{
                Route{ to:_route::HOME, home::home{} }
                Route{ to:_route::MEMO, memo::memo{} }
                Route{ to:_route::NOT_FOUND, not_found::not_found{} }
                Redirect{ from:"", to: _route::NOT_FOUND }
            }
        }
    })
}
