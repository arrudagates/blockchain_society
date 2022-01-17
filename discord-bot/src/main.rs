use std::{collections::HashSet, env::var};

use dotenv::dotenv;
use event_handler::handler;
use primitives::Error;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{id::UserId, prelude::Ready},
    Client,
};

use commands::general::*;

mod commands;
mod event_handler;
mod primitives;

struct Handler;

#[group]
#[commands(ping)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, about: Ready) {
        println!("{} is in!", about.user.name);
    }
}

pub const PREFIX: &str = "&";

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().map_err(Error::Dotenv)?;

    let token = var("DISCORD_TOKEN").map_err(Error::Env)?;
    let app_id = var("APPLICATION_ID")
        .map_err(Error::Env)?
        .parse::<u64>()
        .map_err(Error::ParseInt)?;
    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            owners.insert(UserId::from(319637457907875841));
            match http.get_current_user().await {
                Ok(bot) => (owners, bot.id),
                Err(e) => panic!("Could not access the bot id: {:?}", e),
            }
        }
        Err(e) => panic!("Could not access app info: {:?}", e),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .on_mention(Some(bot_id))
                .ignore_bots(true)
                .ignore_webhooks(true)
                .delimiters([", ", ",", " "])
                .allow_dm(false)
                .with_whitespace(true)
                .prefix(PREFIX)
        })
        .group(&GENERAL_GROUP);

    handler().await?;

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(app_id)
        .await
        .map_err(|e| Error::Client(e.to_string()))?;

    client
        .start()
        .await
        .map_err(|e| Error::Client(e.to_string()))?;

    Ok(())
}
