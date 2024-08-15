use chrono::{DateTime, Utc};
use crate::delivery::api::DeliveryMechanism;
use anyhow::{Result};
use async_trait::async_trait;
use log::{info, warn};
use serde_json::json;
use sqlx::PgPool;
use crate::chat::provider::DebriefResponse;
use crate::read_env_var;

pub struct DbDelivery {}

#[async_trait]
impl DeliveryMechanism for DbDelivery {
    fn get_name(&self) -> String {
        "db".to_string()
    }

    async fn deliver(&self, date_time: &DateTime<Utc>, message: &Vec<DebriefResponse>) ->
                                                                    Result<()> {
        info!("Creating connection pool to postgres");
        let pool = PgPool::connect(&read_env_var("DATABASE_URL")?).await?;

        info!("Inserting message into database");
        let query = sqlx::query!(
            r#"
            INSERT INTO summaries (content, date_time)
            VALUES ($1, $2)
            "#,
            json!(message).to_string(),
            date_time.naive_utc()
        );

        match query.execute(&pool).await {
            Ok(rows_affected) => {
                info!(
                    "Message inserted successfully, rows affected: {}",
                    rows_affected.rows_affected()
                );
            }
            Err(e) => {
                warn!("Could not insert summary into database: {:?}", e);
                return Err(e.into());
            }
        }

        info!("Closing connection pool");
        pool.close().await;

        Ok(())
    }
}