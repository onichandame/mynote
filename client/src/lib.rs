use pages::Loading;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::Layout, routes::Route};

mod components;
mod pages;
mod routes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={Switch::render(switch)}/>
            </Layout>
        </BrowserRouter>
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Redirect<Route> to={Route::Loading}/>},
        Route::Loading => html! {<Loading/>},
    }
}
