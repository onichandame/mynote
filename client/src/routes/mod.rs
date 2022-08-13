use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, Loading, NotFound};

#[derive(Clone, Routable, PartialEq)]
pub enum RootRoute {
    #[at("/")]
    Home,
    #[at("/loading")]
    Loading,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl RootRoute {
    pub fn switch(route: &RootRoute) -> Html {
        match route {
            RootRoute::Home => html! {<Home />},
            RootRoute::Loading => html! {<Loading/>},
            RootRoute::NotFound => html! {<NotFound/>},
        }
    }
}
