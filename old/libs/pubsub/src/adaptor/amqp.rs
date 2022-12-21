use async_trait::async_trait;

use super::PubsubAdaptor;

pub struct AmqpAdaptor {
    conn: lapin::Connection,
}

impl AmqpAdaptor {
    pub async fn create(uri: &str) -> anyhow::Result<Self> {
        let conn = lapin::Connection::connect(uri, lapin::ConnectionProperties::default()).await?;
        Ok(Self { conn })
    }
}

#[async_trait]
impl PubsubAdaptor for AmqpAdaptor {
    async fn publish(&self, topic: &str, msg: &str) -> anyhow::Result<()> {
        let chan = self.conn.create_channel().await?;
        chan.basic_publish("", routing_key, options, payload, properties);
        Ok(())
    }
}
