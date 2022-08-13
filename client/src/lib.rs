use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::Layout, contexts::Global, routes::RootRoute};

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
                    <Switch<RootRoute> render={Switch::render(RootRoute::switch)}/>
                </Layout>
            </BrowserRouter>
        </Global>
    }
}
