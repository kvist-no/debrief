use anyhow::{anyhow, Result};
use delivery::api::DeliveryMechanism;
use log::info;
use octocrab::Octocrab;

mod chat;
mod delivery;
mod github;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise logger to read log level from environment
    env_logger::init();

    // Initialise GitHub SDK
    let instance = configure_octocrab()?;

    // Initialise AI SDK
    let gemini_api_token = read_env_var("GEMINI_API_TOKEN")?;
    let chat_provider = chat::provider::GeminiChatProvider::new(gemini_api_token);

    // Read repository details from environment
    let repository_owner = read_env_var("REPOSITORY_OWNER")?;
    let repository_name = read_env_var("REPOSITORY_NAME")?;

    info!("Github and AI instances created successfully");

    let (date_time, all_pull_requests) =
        github::api::get_merged_pull_requests_from_last_working_day(
            &instance,
            repository_owner.as_str(),
            repository_name.as_str(),
        )
        .await?;
    info!(
        "{} pull requests fetched successfully",
        &all_pull_requests.len()
    );
    let filtered_pull_requests = github::api::filter_out_renovate_pull_requests(all_pull_requests);
    info!(
        "{} pull request(s) left after filtering out dependency updates",
        &filtered_pull_requests.len()
    );

    if filtered_pull_requests.is_empty() {
        info!("No pull requests left after filtering. Exiting early.");
        return Ok(());
    }

    info!("Generating chat response...");
    let chat_response =
        chat::api::generate_brief_summary_of_pull_requests(chat_provider, &filtered_pull_requests)
            .await?;
    info!("Chat response generated successfully");

    info!("Debrief result:\n{:?}", chat_response);

    let delivery_mechanisms = configure_delivery_mechanisms()?;

    for delivery_mechanism in delivery_mechanisms {
        info!(
            "Delivering message using: {}",
            delivery_mechanism.get_name()
        );

        if delivery_mechanism.is_enabled() {
            match delivery_mechanism.deliver(&date_time, &chat_response).await {
                Ok(_) => info!(
                    "Message delivered successfully by {}",
                    delivery_mechanism.get_name()
                ),
                Err(e) => info!("Failed to deliver message: {:?}", e),
            }
        } else {
            info!(
                "Delivery mechanism {} is disabled",
                delivery_mechanism.get_name()
            );
        }
    }

    Ok(())
}

fn read_env_var(var_name: &str) -> Result<String> {
    let err = format!("Missing environment variable: {var_name}");
    match std::env::var(var_name) {
        Ok(val) => Ok(val),
        Err(_) => Err(anyhow!(err)),
    }
}

fn configure_octocrab() -> Result<Octocrab> {
    let github_token = read_env_var("GITHUB_TOKEN")?;
    Ok(Octocrab::builder().personal_token(github_token).build()?)
}

fn configure_delivery_mechanisms() -> Result<Vec<Box<dyn DeliveryMechanism>>> {
    Ok(vec![
        Box::new(delivery::slack::SlackDelivery {}),
        Box::new(delivery::db::DbDelivery {}),
    ])
}
