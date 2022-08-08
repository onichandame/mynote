use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header class={classes!(css!(r#"
            position:fixed;
            top:0;
            left:0;
            right:0;
            z_index:2;
        "#), "mui-appbar", "mui--appbar-line-height")
        }>
            <div class={"mui-container-fluid"}>
                <Link<Route> classes={classes!(css!("text-decoration:none;&:hover{text-decoration:none;}"),"mui--text-display1", "mui--align-middle", "mui--text-light")} to={Route::Home}>
                    {"NoteBook"}
                </Link<Route>>
            </div>
        </header>
    }
}
