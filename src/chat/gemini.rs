use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct Gemini {
    pub api_key: String,
}

impl Gemini {
    pub async fn post(&self, timeout: i32, request: &Request) -> Result<Response> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "https://generativelanguage\
        .googleapis.com/v1beta/models/gemini-1\
        .5-flash:generateContent?key={}",
                self.api_key
            ))
            .json(request)
            .timeout(Duration::from_secs(u64::try_from(timeout).expect(
                "Please use a number in the intersection of u64 and i32",
            )))
            .send()
            .await?;

        response
            .json::<Response>()
            .await
            .map_err(|err| anyhow!(err.to_string()))
    }
}

// API

#[derive(Debug, Serialize)]
pub struct SystemInstructionPart {
    pub text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SystemInstructionContent {
    pub parts: Vec<SystemInstructionPart>,
}

#[derive(Debug, Serialize)]
pub struct GenerationConfig {
    pub response_mime_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
    pub text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RequestContent {
    pub role: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
pub struct Request {
    pub system_instruction: Option<SystemInstructionContent>,
    pub generation_config: Option<GenerationConfig>,
    pub contents: Vec<RequestContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseContent {
    pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Candidate {
    pub content: ResponseContent,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub candidates: Vec<Candidate>,
}
