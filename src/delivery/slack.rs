use log::info;
use crate::delivery::api::DeliveryMechanism;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;

pub struct SlackDelivery {}

#[async_trait]
impl DeliveryMechanism for SlackDelivery {
    async fn deliver(&self, date_time: &DateTime<Utc>, message: &str) -> Result<
        ()> {
        let slack_bot_token = std::env::var("SLACK_API_KEY").expect("Must provide slack API key");
        let slack_channel = std::env::var("SLACK_CHANNEL").expect("Must provide slack channel");

        let client = reqwest::Client::new();

        let response = client.post("https://slack.com/api/chat.postMessage")
            .header("Content-Type", "application/json; charset=utf-8")
            .header("Authorization", format!("Bearer {}", slack_bot_token))
            .json(&ChatPostMessageBody {
                channel: slack_channel,
                blocks: vec![
                    Block {
                        r#type: "header".to_string(),
                        text: Text {
                            r#type: "plain_text".to_string(),
                            text: format!(
                                "Digest for {}",
                                date_time.format("%d/%m/%Y")
                            ).to_string(),
                        },
                    },
                    Block {
                        r#type: "section".to_string(),
                        text: Text {
                            r#type: "mrkdwn".to_string(),
                            text: message.to_string(),
                        },
                    },
                ],
            })
            .send()
            .await?;

        info!("Slack response: {:?}", response.text().await?);

        Ok(())
    }
}

#[derive(Debug, Default, Serialize)]
struct ChatPostMessageBody {
    /// The channel ID to post the message to, e.g. C1234567890
    channel: String,
    blocks: Vec<Block>,
}

#[derive(Debug, Default, Serialize)]
struct Block {
    /// The type of block
    #[serde(rename = "type")]
    r#type: String,
    text: Text,
}

#[derive(Debug, Default, Serialize)]
struct Text {
    /// The type of text
    #[serde(rename = "type")]
    r#type: String,
    text: String,
}