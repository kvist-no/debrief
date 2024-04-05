use octocrab::Octocrab;
use anyhow::Result;
use chatgpt::prelude::*;

mod github;
mod chat;

#[tokio::main]
async fn main() -> Result<()> {
    let github_token = read_env_var("GITHUB_TOKEN");
    let instance = Octocrab::builder().personal_token(github_token).build().expect("Failed to create Octocrab instance");

    let chatgpt_token = read_env_var("OPENAI_TOKEN");
    let chatgpt_instance = ChatGPT::new(chatgpt_token)?;

    let all_pull_requests = github::api::get_merged_pull_requests_from_last_day(&instance, "kvist-no", "frontend").await?;
    let filtered_pull_requests = github::api::filter_out_renovate_pull_requests(all_pull_requests);

    let chat_response = chat::api::generate_brief_summary_of_pull_requests(chatgpt_instance, &filtered_pull_requests).await?;

    println!("{}", chat_response);

    Ok(())
}

fn read_env_var(var_name: &str) -> String {
    let err = format!("Missing environment variable: {var_name}");
    std::env::var(var_name).expect(&err)
}