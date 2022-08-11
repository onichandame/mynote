use yew::prelude::*;

mod client;
mod local_storage;

pub use client::*;

#[function_component(Global)]
pub fn global(props: &GlobalProps) -> Html {
    html! {
        <ClientProvider>
            {props.children.clone()}
        </ClientProvider>

    }
}

#[derive(Properties, PartialEq)]
pub struct GlobalProps {
    pub children: Children,
}
