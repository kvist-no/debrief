use anyhow::{anyhow, Result};
use log::info;
use serde::{Deserialize, Serialize};
use crate::chat::gemini::{Gemini, GenerationConfig, Part, Request, RequestContent, SystemInstructionContent, SystemInstructionPart};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DebriefResponse {
    pub description: String,
    pub url: String,
    pub type_of_change: String,
}

pub trait ChatProvider {
    async fn send_message(
        &self,
        system_instruction: String,
        message: String,
    ) -> Result<Vec<DebriefResponse>>;
}

pub struct GeminiChatProvider {
    client: Gemini,
}

impl GeminiChatProvider {
    pub fn new(api_token: String) -> Self {
        let gemini = Gemini {
            api_key: api_token,
        };

        GeminiChatProvider {
            client: gemini
        }
    }
}

impl ChatProvider for GeminiChatProvider {
    async fn send_message(
        &self,
        system_instruction: String,
        message: String,
    ) -> Result<Vec<DebriefResponse>> {
        let request = Request {
            system_instruction: Some(SystemInstructionContent {
                parts: vec![SystemInstructionPart {
                    text: Some(system_instruction)
                }]
            }),
            generation_config: Some(GenerationConfig {
                response_mime_type: Some("application/json".to_string()),
            }),
            contents: vec![RequestContent {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(message),
                }],
            }],
        };

        let response = self.client.post(30, &request).await?;

        let first_candidate = response.candidates.first();

        match first_candidate.and_then(|candidate| candidate.content.parts.first().and_then(|part| part.text.clone())) {
            Some(text) => {
                match serde_json::from_str::<Vec<DebriefResponse>>(text.as_str()) {
                    Ok(debrief) => {
                        info!("Successfully parsed response from Gemini: {:?}", debrief);
                        Ok(debrief)
                    }
                    Err(e) => {
                        Err(anyhow!("Parsing response from Gemini failed: {:?}", e))
                    }
                }
            }
            None => {
                Err(anyhow!("No valid candidate, part or text parsed as response received from Gemini"))
            }
        }
    }
}