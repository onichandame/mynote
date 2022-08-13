use std::error::Error;

use cynic::{http::SurfExt, MutationBuilder, QueryBuilder};
use yew::prelude::*;

pub use self::schema::queries::{Login, LoginArguments, User, Users};

use super::local_storage::get_local_storage;

mod schema;

static SESSION_KEY: &str = "auth";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Client {
    pub session: Option<String>,
    pub loaded: bool,
}

impl Client {
    pub async fn login(
        &self,
        identity: &str,
        password: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let operation = Login::build(LoginArguments {
            identity: identity.to_owned(),
            password: password.to_owned(),
        });
        let builder = self.request_builder();
        let response = builder.run_graphql(operation).await?;
        Ok(self.unwrap_response(response)?.login)
    }

    pub async fn get_user(&self) -> Result<User, Box<dyn Error + Send + Sync>> {
        let operation = Users::build(());
        let builder = self.request_builder();
        let response = builder.run_graphql(operation).await?;
        Ok(self
            .unwrap_response(response)?
            .users
            .nodes
            .pop()
            .ok_or("user not found")?)
    }

    fn request_builder(&self) -> surf::RequestBuilder {
        let uri = std::option_env!("API_URL").unwrap_or("http://localhost");
        let mut builder = surf::post(uri);
        if let Some(session) = &self.session {
            builder = builder.header("authorization", format!("Bearer {}", session));
        }
        builder
    }

    fn unwrap_response<T>(
        &self,
        response: cynic::GraphQlResponse<T>,
    ) -> Result<T, Box<dyn Error + Send + Sync>> {
        response.data.ok_or_else(|| {
            log::error!("{:?}", &response.errors);
            "login failed".into()
        })
    }
}

#[function_component(ClientProvider)]
pub fn client_provider(props: &ClientProviderProps) -> Html {
    let client = use_state(|| Client::default());
    {
        let client = client.clone();
        use_effect_with_deps(
            move |_| {
                let local_storage = get_local_storage();
                let mut session: Option<String> = None;
                if let Some(saved_session) = local_storage.get_item(SESSION_KEY).unwrap() {
                    session = Some(saved_session.to_owned());
                }
                client.set(Client {
                    session,
                    loaded: true,
                    ..Default::default()
                });
                || ()
            },
            (),
        );
    }
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
            loaded: true,
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
