use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{contexts::use_client, routes::Route};

#[styled_component(Home)]
pub fn home() -> Html {
    let client = use_client();
    html! {
        if client.session.is_some(){
            {"home"}
        }else{
            <Redirect<Route> to={Route::Loading}/>
        }
    }
}
