use chatgpt::prelude::ChatGPT;
use anyhow::Result;

pub async fn generate_brief_summary_of_pull_requests(client: ChatGPT, pull_requests: &Vec<octocrab::models::pulls::PullRequest>) -> Result<String> {
    let mut prompt = "Please add emojis to all subsequent bullet points. You do not need to make the headings bullet points. Bold text with one set of asterisks will suffice. You will be provided with a set of pull requests that have been completed the last 24 hours. I want you to aggregate this into a bullet list highlighting the main changes. You can be a bit technical, but preferably not very. This will be posted to Slack verbatim so keep formatting light. Please attempt to group into areas of responsibility such as 'Bug fixes', 'Reliability', etc. You do not need to include any other text such as 'Here are the changes'. Try to keep the summary to at most 10 lines.\n\n".to_string();

    for pull_request in pull_requests {
        let title = pull_request.clone().title.clone().expect("Title should be set on the pull request");
        let body = pull_request.clone().body.unwrap_or("No body provided".to_string());

        prompt += format!("\n-----\nPull request title: {title}\nPull request body: {body}").as_str();
    }

    let response = client.send_message(prompt).await?;

    Ok(response.message().clone().content)
}
