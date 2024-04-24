use chatgpt::prelude::ChatGPT;
use anyhow::Result;
use log::info;

pub async fn generate_brief_summary_of_pull_requests(client: ChatGPT, pull_requests: &Vec<octocrab::models::pulls::PullRequest>) -> Result<String> {
    let mut prompt = "Please add emojis to all subsequent bullet points to make it a bit more appealing to read. \
    You will be provided with a set of pull requests that have been completed the last 24 hours. \
    I want you to aggregate this into a bullet list highlighting the main changes. \
    You can be a bit technical, but preferably not very. \
    This will be posted to Slack so use the appropriate formatting. \
    Group into sensible categories such as bug fixes, new features, etc. \
    You do not need to include any other text such as 'Here are the changes'. \
    Keep the summary to at most 6 bullet points. \
    Each bullet point should be a human readable sentence. \
    Do not include links to the pull requests, just a brief summary. \
    Do not include these instructions in the output. \
    Here is the content which you should summarise:".to_string();

    for pull_request in pull_requests {
        let title = pull_request.clone().title.clone().expect("Title should be set on the pull request");
        let body = pull_request.clone().body.unwrap_or("No body provided".to_string());

        info!("Adding pull request to prompt: {}", &title);

        prompt += format!("\n-----\nPull request title: {title}\nPull request body: {body}").as_str();
    }

    let response = client.send_message(prompt).await?;

    Ok(response.message().clone().content)
}
