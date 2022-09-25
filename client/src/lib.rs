#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::page::{loading::LoadingPage, welcome::WelcomePage};

mod component;
mod page;

pub fn App(cx: Scope) -> Element {
    rsx!(cx, Router {
        Route { to: "/loading", LoadingPage{} }
        Route {to:"/welcome",WelcomePage{}}
    })
}
