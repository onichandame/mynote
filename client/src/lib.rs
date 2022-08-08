use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};

use pages::loading as Loading;

use crate::components::layout as Layout;

mod components;
mod contexts;
mod pages;

pub fn app(cx: Scope) -> Element {
    rsx!(cx,Layout{
            Router {
                Route{to:"/loading",Loading{}},
                Redirect{from:"/",to:"/loading"},
            }
        }
    )
}
