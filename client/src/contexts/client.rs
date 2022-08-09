use yew::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Client {
    pub session: Option<String>,
}

#[function_component(ClientProvider)]
pub fn client_provider(props: &ClientProviderProps) -> Html {
    let client = use_state(|| Client::default());
    html! {
        <ContextProvider<UseStateHandle<Client>> context={client.clone()}>
            {props.children.clone()}
        </ContextProvider<UseStateHandle<Client>>>
    }
}

pub fn use_session() -> Option<String> {
    let state = use_context::<UseStateHandle<Client>>().expect("client not found");
    (*state).clone().session
}

pub fn use_session_setter() -> Box<dyn Fn(Option<String>)> {
    let state = use_context::<UseStateHandle<Client>>()
        .expect("client not found")
        .clone();
    Box::new(move |sess: Option<String>| {
        state.set(Client { session: sess });
    })
}

#[derive(Properties, PartialEq)]
pub struct ClientProviderProps {
    pub children: Children,
}
