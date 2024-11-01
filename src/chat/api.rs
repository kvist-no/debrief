use anyhow::Result;
use log::info;
use url::Url;
use crate::chat::provider::{ChatProvider, DebriefResponse};

pub async fn generate_brief_summary_of_pull_requests(
    client: impl ChatProvider,
    pull_requests:
    &Vec<octocrab::models::pulls::PullRequest>,
) -> Result<Vec<DebriefResponse>> {
    let prompt = r#"You will be provided with a set of pull requests that have been completed the working day.
    Avoid using technical language where possible.
    The PR title should be stripped of prefixes such as 'refactor:',
    'fro-123:', etc. and rewritten in a human readable way. Capitalize the first letter of the title.
    Each description must be at most 70 characters.
    Each line should be a human readable sentence. Rewrite it if necessary.
    Do not prefix the lines with hyphens.
    The entire message must be at most 1500 characters, if the message is too long, skip the least important changes.
    The type of change should be a capitalized string such as 'Feature', 'Bug
     fix', 'Refactor', etc. Add an emoji to the beginning of the type of change.
    Output the result using this JSON schema:
    { "type": "array"
    , "items": { "type": "object"
               , "properties": { "description": { "type": "string" }
                               , "url": { "type": "string" }
                               , "type_of_change": { "type": "string" }
                               }
               }
    }
    "#.to_string();

    let mut input = "".to_string();

    for pull_request in pull_requests {
        let title = pull_request.clone().title.clone().unwrap_or("No title provided".to_string());
        let body = pull_request.clone().body.unwrap_or("No body provided".to_string());
        let url = pull_request.clone().html_url.unwrap_or(Url::parse("https://github.com").unwrap());

        info!("Adding pull request to prompt: {}", &title);

        input += format!("Pull request title: {title}\nPull request \
        URL: {url}\nPull request body: {body}\n-----\n").as_str();
    }

    let response = client.send_message(prompt, input).await?;

    Ok(response)
}
