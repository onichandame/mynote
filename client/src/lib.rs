use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::Layout,
    contexts::Global,
    pages::{Home, Loading, NotFound},
    routes::Route,
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
                    <Switch<Route> render={Switch::render(switch)}/>
                </Layout>
            </BrowserRouter>
        </Global>
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Loading => html! {<Loading/>},
        Route::NotFound => html! {<NotFound/>},
    }
}
