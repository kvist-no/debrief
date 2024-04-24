use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait DeliveryMechanism {
    async fn deliver(&self, message: &str) -> Result<()>;
}