use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::Layout,
    contexts::Global,
    pages::{Home, Loading, NotFound},
    routes::RootRoute,
};

mod components;
mod contexts;
mod pages;
mod routes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Global>
            <BrowserRouter>
                <Layout>
                    <Switch<RootRoute> render={Switch::render(switch)}/>
                </Layout>
            </BrowserRouter>
        </Global>
    }
}

fn switch(route: &RootRoute) -> Html {
    match route {
        RootRoute::Home => html! {<Home />},
        RootRoute::Loading => html! {<Loading/>},
        RootRoute::NotFound => html! {<NotFound/>},
    }
}
