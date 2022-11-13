use std::{collections::HashMap, pin::Pin};

use async_stream::stream;
use async_trait::async_trait;
use futures::Stream;
use rand::seq::SliceRandom;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::PubsubRequestConfig;

use super::PubsubAdaptor;

pub struct MockAdaptor {
    subscriptions: HashMap<String, HashMap<String, Vec<mpsc::Sender<String>>>>,
}

impl MockAdaptor {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }
}

#[async_trait]
impl PubsubAdaptor for MockAdaptor {
    async fn publish(&self, topic: &str, msg: &str) -> anyhow::Result<()> {
        if let Some(subscribers) = self.subscriptions.get(topic) {
            for (_, subscribers) in subscribers.iter() {
                if let Some(subscriber) = subscribers.choose(&mut rand::rngs::OsRng::default()) {
                    subscriber.send(msg.to_owned()).await?;
                }
            }
        }
        Ok(())
    }

    async fn subscribe(
        &mut self,
        topic: &str,
        conf: Option<PubsubRequestConfig>,
    ) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        let (sender, mut recver) = mpsc::channel(8);
        let group = if let Some(conf) = conf {
            if let Some(group) = conf.group {
                group
            } else {
                Uuid::new_v4().to_string()
            }
        } else {
            Uuid::new_v4().to_string()
        };
        if self.subscriptions.get(topic).is_none() {
            self.subscriptions.insert(topic.to_owned(), HashMap::new());
        }
        if let Some(subs) = self.subscriptions.get_mut(topic) {
            if subs.get(&group).is_none() {
                subs.insert(group.clone(), vec![]);
            }
            if let Some(subs) = subs.get_mut(&group) {
                subs.push(sender);
            }
        }
        Box::pin(stream! {
            while let Some(pld) = recver.recv().await{
                yield pld;
            }
        })
    }
}
