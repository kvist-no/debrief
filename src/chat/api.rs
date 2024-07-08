use chatgpt::prelude::ChatGPT;
use anyhow::Result;
use log::info;
use url::Url;

pub async fn generate_brief_summary_of_pull_requests(client: ChatGPT, pull_requests: &Vec<octocrab::models::pulls::PullRequest>) -> Result<String> {
    let mut prompt = "Please add emojis to all subsequent bullet points based on what type of change it is. \
    You will be provided with a set of pull requests that have been completed the working day. \
    I want you to aggregate this into a bullet list highlighting the main changes. \
    Avoid using technical language where possible. \
    This will be posted to Slack so use the appropriate formatting. \
    Group the changes by the PR title which introduced them. \
    The PR title should be stripped of prefixes such as 'refactor:', 'fro-123:', etc. and formatted for legibility \
    Example: *<PR url|PR title>* followed by a newline and a short sentence explaining the changes introduced. \
    You do not need to include any other text such as 'Here are the changes'. \
    Each bullet point should be at most 70 characters to keep it concise. \
    Each bullet point should be a human readable sentence. \
    The entire message must be at most 1500 characters. \
    The PR header should not be a bullet point. \
    Do not use '-' to indicate a bullet point. This is done by the emojis. Do not include a ':' after the emoji. \
    Do not include these instructions in the output. \
    Here is the content which you should summarise:".to_string();

    for pull_request in pull_requests {
        let title = pull_request.clone().title.clone().expect("Title should be set on the pull request");
        let body = pull_request.clone().body.unwrap_or("No body provided".to_string());
        let url = pull_request.clone().html_url.unwrap_or(Url::parse("https://github.com").unwrap());

        info!("Adding pull request to prompt: {}", &title);

        prompt += format!("\n-----\nPull request title: {title}\nPull request URL: {url}\nPull request body: {body}").as_str();
    }

    let response = client.send_message(prompt).await?;

    Ok(response.message().clone().content)
}
