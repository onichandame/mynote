use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class={classes!(css!(""),"mui--text-center")}>
            {"loading"}
        </div>
    }
}
