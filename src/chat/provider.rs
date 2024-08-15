use anyhow::{anyhow, Result};
use google_generative_ai_rs::v1::api::Client;
use google_generative_ai_rs::v1::gemini::{Content, Model, Part, Role};
use google_generative_ai_rs::v1::gemini::request::{GenerationConfig, Request, SystemInstructionContent, SystemInstructionPart};
use log::info;
use serde::{Deserialize, Serialize};

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
    client: Client,
}

impl GeminiChatProvider {
    pub fn new(api_token: String) -> Self {
        let client = Client::new_from_model(Model::Gemini1_5Pro, api_token);

        GeminiChatProvider {
            client
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
            tools: vec![],
            generation_config: Some(GenerationConfig {
                temperature: None,
                top_p: None,
                top_k: None,
                candidate_count: None,
                max_output_tokens: None,
                stop_sequences: None,
                response_mime_type: Some("application/json".to_string()),
            }),
            contents: vec![Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(message),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            }],
            safety_settings: vec![]
        };

        let response = self.client.post(30, &request).await?;

        match response.rest() {
            Some(rest) => {
                let first_candidate = rest.candidates.first();

                match first_candidate.and_then(|candidate|  candidate.content.parts.first().and_then(|part| part.text.clone())) {
                    Some(text) => {
                        match serde_json::from_str::<Vec<DebriefResponse>>(text.as_str()) {
                            Ok(debrief) => {
                                info!("Successfully parsed response from Gemini: {:?}", debrief);
                                Ok(debrief)
                            }
                            Err(e) => {
                                Err(anyhow!("Error parsing response from Gemini: {:?}", e))
                            }
                        }
                    }
                    None => {
                        Err(anyhow!("No valid candidate, part or text parsed as response received from Gemini"))
                    }
                }
            }
            None => {
                Err(anyhow!("Error generating candidates from Gemini"))
            },
        }
    }
}