# Debrief — AI generated daily development updates

<img width="720" height="177" alt="example of use" src="https://github.com/user-attachments/assets/078a6a12-a123-4977-8fd9-260d23f4a1a2" />

At Kvist we faced the need for receiving daily updates on the development of our projects. We wanted to have a clear overview of what was done without having to go through commits and pull requests.

From this need, we created Debrief which is a simple Docker image you can run as a cronjob in your Kubernetes cluster or wherever to receive a message in Slack every day containing the changes done the previous day (this accounts for weekends by sending updates on things done Friday through Sunday).

## How to use it

This project uses Taskfile, and you can use `task run` and `task build` to run and build the project. It will read from `.env` so create that file and populate it with the desired values.

This tool is built around the concept of delivery mechanisms which is just a
way to list up where you want the daily updates to be sent. Currently, we support Slack and a database delivery mechanism.

You can enable the different ones with the following environment variables:

```env
DELIVERY_SLACK_ENABLED=true
DELIVERY_DB_ENABLED=true
```

Not setting them, or setting them to false, disables them.

---

Run the Docker image with the following environment variables:

```env
# A GitHub PAT which has access to read the pull requests of the repository. A fine-grained will do.
GITHUB_TOKEN=
# A Gemini API token with access to Pro 1.5.
GEMINI_API_TOKEN=
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
# The database URL if you want to use the DB delivery mechanism
DATABASE_URL=
```
