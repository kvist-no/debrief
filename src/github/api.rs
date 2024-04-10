use octocrab::Octocrab;
use octocrab::params::pulls::Sort;
use octocrab::params::{Direction, State};
use anyhow::Result;
use octocrab::models::pulls::PullRequest;

const RUNS_AT_TIME_OF_DAY: i64 = 0;

pub async fn get_merged_pull_requests_from_last_day(instance: &Octocrab, organisation: &str, repository: &str) -> Result<Vec<PullRequest>> {
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
                if merged_at > chrono::Utc::now() - chrono::Duration::days(1) - chrono::Duration::hours(RUNS_AT_TIME_OF_DAY) && merged_at < chrono::Utc::now() - chrono::Duration::hours(RUNS_AT_TIME_OF_DAY) {
                    merged_pull_requests.push(pull.clone());
                }
            }
            None => continue
        }
    }

    Ok(merged_pull_requests)
}

pub fn filter_out_renovate_pull_requests(pull_requests: Vec<PullRequest>) -> Vec<PullRequest> {
    pull_requests.into_iter().filter(|pull| {
        pull.user.as_ref().unwrap().login != "dependabot[bot]" && pull.user.as_ref().unwrap().login != "renovate[bot]"
    }).collect()
}