use yew::prelude::*;

use super::local_storage::get_local_storage;

static SESSION_KEY: &str = "auth";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Client {
    pub session: Option<String>,
}

#[function_component(ClientProvider)]
pub fn client_provider(props: &ClientProviderProps) -> Html {
    let client = use_state(|| Client::default());
    use_effect_with_deps(
        move |client| {
            let local_storage = get_local_storage();
            if let Some(saved_session) = local_storage.get_item(SESSION_KEY).unwrap() {
                client.set(Client {
                    session: Some(saved_session.to_owned()),
                });
            }
            || ()
        },
        client.clone(),
    );
    html! {
        <ContextProvider<UseStateHandle<Client>> context={client.clone()}>
            {props.children.clone()}
        </ContextProvider<UseStateHandle<Client>>>
    }
}

pub fn use_client() -> Client {
    let state = use_context::<UseStateHandle<Client>>().expect("client not found");
    (*state).clone()
}

pub fn use_session_setter() -> Box<dyn Fn(Option<String>)> {
    let state = use_context::<UseStateHandle<Client>>()
        .expect("client not found")
        .clone();
    Box::new(move |sess: Option<String>| {
        let local_storage = get_local_storage();
        state.set(Client {
            session: sess.clone(),
        });
        if let Some(sess) = sess {
            local_storage.set_item(SESSION_KEY, &sess).unwrap();
        } else {
            local_storage.remove_item(SESSION_KEY).unwrap();
        }
    })
}

#[derive(Properties, PartialEq)]
pub struct ClientProviderProps {
    pub children: Children,
}
