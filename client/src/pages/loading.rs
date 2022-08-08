use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class={classes!(css!("border: solid;border-radius: 5px;"))}>{"loading"}</div>
    }
}
