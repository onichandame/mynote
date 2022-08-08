use stylist::yew::styled_component;
use yew::prelude::*;

use self::navbar::Navbar;

mod navbar;

#[styled_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html! {
        <div>
            <Navbar/>
            <div class={"mui--appbar-height"} />
            <div class={classes!(css!("margin-top: 1rem;"),"mui-container-fluid")}>
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}
