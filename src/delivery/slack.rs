use std::string::ToString;
use log::info;
use crate::delivery::api::DeliveryMechanism;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::chat::provider::DebriefResponse;
use crate::read_env_var;
use itertools::Itertools;

pub struct SlackDelivery {}

#[async_trait]
impl DeliveryMechanism for SlackDelivery {
    fn get_name(&self) -> String {
        "slack".to_string()
    }

    async fn deliver(
        &self,
        date_time: &DateTime<Utc>,
        debrief:
        &Vec<DebriefResponse>,
    ) -> Result<()> {
        let slack_bot_token = read_env_var("SLACK_API_KEY")?;
        let slack_channel = read_env_var("SLACK_CHANNEL")?;

        let client = reqwest::Client::new();

        let message = generate_slack_message(debrief);

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
                            text: message
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

impl DebriefResponse {
    pub fn to_slack_message(&self) -> String {
        format!(
            "<{}|{}>",
            self.url,
            self.description,
        )
    }
}

fn generate_slack_message(debriefs: &Vec<DebriefResponse>) -> String {
    debriefs.into_iter().into_group_map_by(|debrief| {
        debrief.type_of_change.clone()
    }).into_iter().map(|(group, items)| {
        format!(
            "*{}*\n{}",
            group,
            items.into_iter().map(|item| item.to_slack_message()).join("\n")
        )
    }).join("\n")
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