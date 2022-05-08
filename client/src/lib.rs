use async_trait::async_trait;
use futures::{Stream, StreamExt};
use note::StreamNoteOutput;
use password::StreamPasswordOutput;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tokio_graphql_ws::{ClientTrait, Request, Subscriber};

mod login;
mod note;
mod password;

pub use crate::login::*;
pub use crate::password::*;

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
        #[derive(Serialize, Default)]
        struct Input {
            input: LoginInput,
        }
        Ok(self
            .request::<LoginOutput, _>(
                "
            mutation($input:LoginInput!){
                login(input:$input)
            }",
                Input { input },
            )
            .await?
            .next()
            .await
            .ok_or("failed to login")??
            .login)
    }

    pub async fn stream_notes(
        &self,
    ) -> Result<impl Stream<Item = Result<model::note::Model, Error>> + '_, Error> {
        self.login_required()?;
        Ok(Box::pin(
            self.request::<StreamNoteOutput, _>(
                "
            subscription{
                streamNotes{
                    id
                    uuid
                    createdAt
                    updatedAt
                    deletedAt
                    userId
                    title
                    content
                }
            }
            ",
                (),
            )
            .await?
            .map(|v| v.map(|v| v.stream_notes)),
        ))
    }

    pub async fn stream_passwords(
        &self,
        filter: Option<PasswordFilter>,
    ) -> Result<impl Stream<Item = Result<model::password::Model, Error>> + '_, Error> {
        #[derive(Serialize, Default)]
        pub struct Input {
            pub filter: Option<PasswordFilter>,
        }
        self.login_required()?;
        Ok(Box::pin(
            self.request::<StreamPasswordOutput, _>(
                "
            subscription($filter:PasswordFilter){
                streamPasswords(filter:$filter){
                    id
                    uuid
                    createdAt
                    updatedAt
                    deletedAt
                    isLocal
                    userId
                    groupId
                    title
                    password
                    url
                    username
                    email
                }
            }
            ",
                Input { filter },
            )
            .await?
            .map(|v| v.map(|v| v.stream_passwords)),
        ))
    }

    async fn request<TOut: DeserializeOwned, TIn: Serialize + Sync + Default>(
        &self,
        query: &str,
        input: TIn,
    ) -> Result<impl Stream<Item = Result<TOut, Error>> + '_, Error> {
        Ok(self
            .subscribe(&Request::<TIn, ()> {
                query: query.to_owned(),
                variables: input,
                ..Default::default()
            })
            .await?
            .map(|v| match v {
                Ok(v) => v.errors.map_or(
                    match v.data {
                        Value::Null => Err("no data received".into()),
                        other => Ok(serde_json::from_value::<TOut>(other)?),
                    },
                    |v| {
                        Err(v
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join("; ")
                            .into())
                    },
                ),
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
