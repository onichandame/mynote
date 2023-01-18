use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};

mod component;
mod page;
mod provider;

use crate::app::{
    component::{icon, layout},
    page::{_route, home, memo, not_found},
};

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        style{
            vec![include_str!("../../gen/app.css")].into_iter().map(|v|rsx!(v))
        }
        link { rel:"stylesheet", href:"https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css" }
        icon::init_icons{}
        provider::provider{
            Router{
                layout::layout{
                    Route{ to:_route::HOME, home::home{} }
                    Route{ to:_route::MEMOS, memo::memo{} }
                    Route{ to:_route::CREATE_MEMO, memo::create{} }
                    Route{ to:_route::NOT_FOUND, not_found::not_found{} }
                    Redirect{ from:"", to: _route::NOT_FOUND }
                }
            }
        }
    })
}
