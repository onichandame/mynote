use pages::Loading;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::Layout, contexts::ClientProvider, routes::Route};

mod components;
mod contexts;
mod pages;
mod routes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <ClientProvider>
                <Layout>
                    <Switch<Route> render={Switch::render(switch)}/>
                </Layout>
            </ClientProvider>
        </BrowserRouter>
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Redirect<Route> to={Route::Loading}/>},
        Route::Loading => html! {<Loading/>},
    }
}
