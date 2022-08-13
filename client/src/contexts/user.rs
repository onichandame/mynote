use yew::prelude::*;

use super::{use_client, User};

pub fn user_provider() -> Html {
    let user = use_state(|| Option::<User>::None);
    let client = use_client();
    use_effect_with_deps(move |client| || (), client.clone());
    html! {
        <ContextProvider<UseStateHandle<User>> context={user.clone()}>
        </ContextProvider<UseStateHandle<User>>>
    }
}
