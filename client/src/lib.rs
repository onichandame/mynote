use async_trait::async_trait;
use futures::{Stream, StreamExt};
use schema::SCEHAM;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tokio_graphql_ws::{ClientTrait, Request, Subscriber};

mod login;

pub use crate::login::*;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Default, Clone, Serialize, Subscriber)]
#[graphql_ws(url = "self.url.clone()")]
pub struct Client {
    session: Option<String>,
    #[serde(skip)]
    url: String,
}

impl Client {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
            ..Default::default()
        }
    }

    pub fn set_session(self, sess: &str) -> Self {
        Self {
            session: Some(sess.to_owned()),
            ..self
        }
    }

    pub async fn login(&self, input: LoginInput) -> Result<String, Error> {
        Ok(self
            .request::<LoginOutput, _>(input, "login")
            .await?
            .next()
            .await
            .ok_or("failed to login")??
            .login)
    }

    pub async fn stream_notes(&self) -> Result<impl Stream<Item = model::note::Model> + '_, Error> {
        self.login_required()?;
        Ok(Box::pin(
            self.request::<model::note::Model, _>((), "streamNotes")
                .await?
                .filter_map(|v| async move { v.ok() }),
        ))
    }

    async fn request<TOut: DeserializeOwned, TIn: Serialize + Sync + Default>(
        &self,
        input: TIn,
        operation_name: &str,
    ) -> Result<impl Stream<Item = Result<TOut, Error>> + '_, Error> {
        Ok(self
            .subscribe(&Request::<TIn, ()> {
                query: SCEHAM.to_owned(),
                operation_name: Some(operation_name.to_owned()),
                variables: input,
                ..Default::default()
            })
            .await?
            .map(|v| match v {
                Ok(v) => Ok(serde_json::from_value::<TOut>(v.data)?),
                Err(e) => Err(e.to_string().into()),
            }))
    }

    fn login_required(&self) -> Result<(), Error> {
        Ok(self.session.as_ref().map(|_| ()).ok_or("not logged in")?)
    }
}

#[async_trait]
impl ClientTrait for Client {
    async fn connection_init(&self) -> Result<Option<Value>, Error> {
        Ok(Some(serde_json::to_value(self)?))
    }
}
