use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait DeliveryMechanism {
    async fn deliver(
        &self,
        date_time: &DateTime<Utc>,
        message: &str,
    ) -> Result<()>;
}