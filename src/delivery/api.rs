use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::read_env_var;

#[async_trait]
pub trait DeliveryMechanism {
    fn get_name(&self) -> String;

    fn is_enabled(&self) -> bool {
        read_env_var(format!("DELIVERY_{}_ENABLED", self.get_name().to_uppercase()).as_str())
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap_or(false)
    }

    async fn deliver(
        &self,
        date_time: &DateTime<Utc>,
        message: &str,
    ) -> Result<()>;
}