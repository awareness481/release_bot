use dotenv::dotenv;
use octocrab::models::events::payload::EventPayload;
use octocrab::models::events::EventType::UnknownEvent;
mod md;
mod types;
use octocrab::Octocrab;
use octocrab::{etag::Etagged, models::events::Event, Page};
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::collections::VecDeque;
use std::env;
use tokio::sync::mpsc::{self, Sender};

const DELAY_MS: u64 = 500;
const TRACKING_CAPACITY: usize = 20;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let channel_id = std::env::var("CHANNEL_ID").expect("CHANNEL_ID not set");

        let (tx, mut rx) = mpsc::channel(100);
        tokio::spawn(async move {
            crab(&tx).await.unwrap();
        });

        while let Some(res) = rx.recv().await {
            ChannelId(channel_id.parse().unwrap())
                .send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title(res.release.tag_name)
                            .description(md::parse_string(&res.release.body.unwrap()))
                            .color(0x00ff00)
                    })
                })
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        panic!("Client error: {:?}", why);
    }
}

async fn crab(s: &Sender<Release>) -> octocrab::Result<()> {
    let mut etag = None;
    let mut seen = VecDeque::with_capacity(TRACKING_CAPACITY);

    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let repo_owner = env::var("REPO_OWNER").expect("REPO_OWNER env variable is required");
    let repo_name = env::var("REPO_NAME").expect("REPO_NAME env variable is required");

    let octo = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    loop {
        let response: Etagged<Page<Event>> = octo
            .repos(repo_owner.to_string(), repo_name.to_string())
            .events()
            .etag(etag)
            .per_page(10)
            .send()
            .await?;
        if let Some(page) = response.value {
            for event in page {
                // If an etag changes and we get a new page, this page may contain events we have
                // already seen along with new events. So, keep track of the ones we have seen for
                // each page, this will be at most 20 events - the current page of 10 events and
                // the last page.
                if !seen.contains(&event.id) {
                    println!(
                        "New event : id = {:?}, type = {:?}, time = {:?}",
                        event.id, event.r#type, event.created_at
                    );
                    if event.r#type == UnknownEvent("ReleaseEvent".to_string()) {
                        let p = &event.payload.unwrap();
                        let c: Release = p.into();
                        s.send(c).await.unwrap();
                    }
                    if seen.len() == TRACKING_CAPACITY {
                        seen.pop_back();
                    }
                    seen.push_front(event.id);
                }
            }
        }
        etag = response.etag;
        tokio::time::sleep(tokio::time::Duration::from_millis(DELAY_MS)).await;
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
struct Release {
    pub action: String,
    pub release: types::Release,
}

impl From<&EventPayload> for Release {
    fn from(payload: &EventPayload) -> Self {
        match payload {
            EventPayload::UnknownEvent(payload) => {
                dbg!(&payload);
                return serde_json::from_value(payload.to_owned().into_deserializer()).unwrap();
            }
            _ => panic!("unexpected event payload encountered: {:#?}", payload),
        }
    }
}
