use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(NotFound)]
pub fn not_found() -> Html {
    html! {"404 | page not found"}
}
