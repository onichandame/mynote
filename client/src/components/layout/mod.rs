use stylist::yew::styled_component;
use yew::prelude::*;

use crate::contexts::use_client;

use self::navbar::Navbar;

mod navbar;

#[styled_component(Layout)]
pub fn layout(props: &Props) -> Html {
    let client = use_client();
    html! {
        <div>
            <Navbar/>
            <div class={"mui--appbar-height"} />
            <div class={classes!(css!("margin-top: .5rem;"),"mui-container-fluid")}>
                if client.loaded{
                    {props.children.clone()}
                }else{
                    {"loading"}
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}
