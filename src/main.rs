use octocrab::Octocrab;
use anyhow::Result;

mod github;

#[tokio::main]
async fn main() -> Result<()> {
    let token = read_env_var("GITHUB_TOKEN");

    let instance = Octocrab::builder().personal_token(token).build().expect("Failed to create Octocrab instance");

    let all_pull_requests = github::api::get_merged_pull_requests_from_last_day(&instance, "kvist-no", "frontend").await?;
    let filtered_pull_requests = github::api::filter_out_renovate_pull_requests(all_pull_requests);

    for pull in filtered_pull_requests {
        println!("@{} merged #{}: {}", pull.user.expect("User should be set").login, pull.number, pull.title.expect("Title should be set"));
    }

    Ok(())
}

fn read_env_var(var_name: &str) -> String {
    let err = format!("Missing environment variable: {var_name}");
    std::env::var(var_name).expect(&err)
}