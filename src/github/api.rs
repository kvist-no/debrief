use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, Timelike, Utc, Weekday};
use log::info;
use octocrab::models::pulls::PullRequest;
use octocrab::params::pulls::Sort;
use octocrab::params::{Direction, State};
use octocrab::Octocrab;

pub async fn get_merged_pull_requests_from_last_working_day(
    instance: &Octocrab,
    organisation: &str,
    repository: &str,
) -> Result<(DateTime<Utc>, Vec<PullRequest>)> {
    let beginning_datetime = get_beginning_of_last_working_day();
    info!(
        "Getting all PRs merged after: {}",
        beginning_datetime.to_rfc2822()
    );

    let page = instance
        .pulls(organisation, repository)
        .list()
        .state(State::Closed)
        .sort(Sort::Updated)
        .per_page(50)
        .direction(Direction::Descending)
        .send()
        .await?;

    let mut merged_pull_requests = Vec::new();

    for pull in page.items {
        match pull.merged_at {
            Some(merged_at) => {
                if merged_at > beginning_datetime {
                    merged_pull_requests.push(pull.clone());
                }
            }
            None => continue,
        }
    }

    Ok((beginning_datetime, merged_pull_requests))
}

pub fn filter_out_renovate_pull_requests(pull_requests: Vec<PullRequest>) -> Vec<PullRequest> {
    pull_requests
        .into_iter()
        .filter(|pull| {
            pull.user.as_ref().unwrap().login != "dependabot[bot]"
                && pull.user.as_ref().unwrap().login != "renovate[bot]"
        })
        .collect()
}

fn get_beginning_of_last_working_day() -> DateTime<Utc> {
    let last_working_day = get_last_working_day();
    last_working_day
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}

fn get_last_working_day() -> DateTime<Utc> {
    match Utc::now().weekday() {
        Weekday::Mon => Utc::now() - Duration::days(3),
        _ => Utc::now() - Duration::days(1),
    }
}
