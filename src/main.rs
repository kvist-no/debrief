use std::{time};
use octocrab::Octocrab;
use anyhow::Result;
use chatgpt::prelude::*;
use log::{info};
use delivery::api::DeliveryMechanism;

mod github;
mod chat;
mod delivery;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logger to read log level from environment
    env_logger::init();

    // Initialise GitHub SDK
    let instance = configure_octocrab();

    // Initialise ChatGPT SDK
    let chatgpt_instance = configure_chatgpt()?;

    // Read repository details from environment
    let repository_owner = read_env_var("REPOSITORY_OWNER");
    let repository_name = read_env_var("REPOSITORY_NAME");

    info!("Github and ChatGPT instances created successfully");

    let all_pull_requests = github::api::get_merged_pull_requests_from_last_working_day(&instance, repository_owner.as_str(), repository_name.as_str()).await?;
    info!("{} pull requests fetched successfully", &all_pull_requests.len());
    let filtered_pull_requests = github::api::filter_out_renovate_pull_requests(all_pull_requests);
    info!("{} pull request(s) left after filtering out dependency updates", &filtered_pull_requests.len());

    if &filtered_pull_requests.is_empty() {
        info!("No pull requests left after filtering. Exiting early.");
        return Ok(());
    }

    info!("Generating chat response...");
    let chat_response = chat::api::generate_brief_summary_of_pull_requests(chatgpt_instance, &filtered_pull_requests).await?;
    info!("Chat response generated successfully");

    info!("Debrief result:\n{}", chat_response);

    let delivery_mechanism = configure_delivery_mechanism()?;

    delivery_mechanism.deliver(&chat_response).await?;

    Ok(())
}

fn read_env_var(var_name: &str) -> String {
    let err = format!("Missing environment variable: {var_name}");
    std::env::var(var_name).expect(&err)
}

fn configure_octocrab() -> Octocrab {
    let github_token = read_env_var("GITHUB_TOKEN");
    Octocrab::builder().personal_token(github_token).build().expect("Failed to create Octocrab instance")
}

fn configure_chatgpt() -> Result<ChatGPT> {
    let chatgpt_token = read_env_var("OPENAI_TOKEN");
    Ok(ChatGPT::new_with_config(
        chatgpt_token,
        // We want to use GPT-4 with an increased timeout as we're passing quite a lot of data
        ModelConfigurationBuilder::default().engine(ChatGPTEngine::Gpt4).timeout(time::Duration::from_secs(60)).build()?,
    )?)
}

fn configure_delivery_mechanism() -> Result<Box<impl DeliveryMechanism>> {
    Ok(Box::new(delivery::slack::SlackDelivery {}))
}