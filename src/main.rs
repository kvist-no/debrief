use std::time;
use octocrab::Octocrab;
use anyhow::Result;
use chatgpt::prelude::*;
use log::{info};

mod github;
mod chat;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let github_token = read_env_var("GITHUB_TOKEN");
    let instance = Octocrab::builder().personal_token(github_token).build().expect("Failed to create Octocrab instance");

    let chatgpt_token = read_env_var("OPENAI_TOKEN");
    let chatgpt_instance = ChatGPT::new_with_config(
        chatgpt_token,
        ModelConfigurationBuilder::default().engine(ChatGPTEngine::Gpt4).timeout(time::Duration::from_secs(30)).build()?,
    )?;

    info!("Github and ChatGPT instances created successfully");

    let all_pull_requests = github::api::get_merged_pull_requests_from_last_day(&instance, "kvist-no", "frontend").await?;
    info!("{} pull requests fetched successfully", &all_pull_requests.len());
    let filtered_pull_requests = github::api::filter_out_renovate_pull_requests(all_pull_requests);
    info!("{} pull request(s) left after filtering out dependency updates", &filtered_pull_requests.len());

    info!("Generating chat response...");
    let chat_response = chat::api::generate_brief_summary_of_pull_requests(chatgpt_instance, &filtered_pull_requests).await?;
    info!("Chat response generated successfully");

    println!("{}", chat_response);

    Ok(())
}

fn read_env_var(var_name: &str) -> String {
    let err = format!("Missing environment variable: {var_name}");
    std::env::var(var_name).expect(&err)
}