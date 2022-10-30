A bot to post github releases of a repo to your discord channel

### Instructions

1. Clone the repo `git clone https://github.com/awareness481/release_bot`
2. `mv .env.sample .env` and fill the necessary values
   - `DISCORD_TOKEN` is the token you get when creating a bot
     - Grant "Server members intent" permission
     - Grant "Message content intent" permission
   - `GITHUB_TOKEN` https://github.com/settings/tokens No special permissions
   - `CHANNEL_ID` ID of the channel you want the release messages
   - `REPO_OWNER` & `REPO_NAME` In `https://github.com/microsoft/TypeScript` the `REPO_OWNER` would be `microsoft`  
      and the `REPO_NAME` would be `TypeScript`
3. `cargo run` for dev or `cargo run --release` for prod
