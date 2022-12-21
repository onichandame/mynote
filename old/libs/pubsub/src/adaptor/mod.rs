use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;

use super::PubsubRequestConfig;

mod amqp;
mod mock;

pub use mock::MockAdaptor;

#[async_trait]
pub trait PubsubAdaptor {
    async fn publish(&self, topic: &str, msg: &str) -> anyhow::Result<()>;
    async fn subscribe(
        &mut self,
        topic: &str,
        conf: Option<PubsubRequestConfig>,
    ) -> Pin<Box<dyn Stream<Item = String> + Send>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use futures::StreamExt;
    macro_rules! adaptor_tests {
        ($($name:ident: $adaptor:expr,)*) => {
            $(
                mod $name{
                    use super::*;
                    #[tokio::test]
                    async fn can_pubsub()->anyhow::Result<()>{
                        let mut adaptor=$adaptor;
                        let topic="test";
                        let msg="test";
                        let mut subscription=adaptor.subscribe(topic,None).await;
                        let sub1=tokio::spawn(async move{
                            subscription.next().await
                        });
                        let mut subscription=adaptor.subscribe(topic,None).await;
                        let sub2=tokio::spawn(async move{
                            subscription.next().await
                        });
                        adaptor.publish(topic,msg).await?;
                        assert_eq!(sub1.await?.context("failed to receive msg")?,msg);
                        assert_eq!(sub2.await?.context("failed to receive msg")?,msg);
                        Ok(())
                    }
                }
            )*
        };
    }

    adaptor_tests! {mock: MockAdaptor::new(),}
}
