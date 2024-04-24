# Debrief — AI assisted daily development updates

Hello. At Kvist we faced the need for receiving daily updates on the development of our projects. We wanted to have a clear overview of what was done without having to go through commits and pull requests.

From this need, we created Debrief which is a simple Docker image you can run as a cronjob in your Kubernetes cluster or wherever to receive a message in Slack every day containing the changes done the previous day (this accounts for weekends by sending updates on things done Friday through Sunday).

## How to use it

Run the Docker image with the following environment variables:

```env
# A GitHub PAT which has access to read the pull requests of the repository. A fine-grained will do.
GITHUB_TOKEN=
# An OpenAI token with access to GPT-4. We tried GPT-3.5-turbo, but it didn't work that well.
OPENAI_TOKEN=
# The GitHub repository name
REPOSITORY_NAME=
# GitHub organization name or username in the case you want to use this for your own projects
REPOSITORY_OWNER=
# We use env_logger in Rust so you can use this variable to control the log level. We recommend info in the case you need to debug.
RUST_LOG=info
# The Slack bot token. You only need chat:write permissions given that you invite the bot user to the channel you want to write to.
SLACK_API_KEY=
# The Slack channel ID you want to deliver the message to
SLACK_CHANNEL=
```