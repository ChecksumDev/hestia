mod commands;
mod events;
pub mod groups;
pub mod hooks;
mod utils;

use std::collections::HashSet;
use std::sync::Arc;

use commands::help::HELP;
use groups::{DEV_GROUP, MISC_GROUP, STAFF_GROUP, USER_GROUP};

use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::{client::bridge::gateway::ShardManager, framework::standard::buckets::LimitedFor};

use serenity::prelude::*;
use tracing::error;

use hooks::{after, before, delay_action, dispatch_error, normal_message, unknown_command};
use utils::config::Config;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler; // For use in events/

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // Initialize tracing

    let config = Config::from_toml("config.toml").unwrap();
    let http = Http::new(config.token());

    // Get the owners of the bot from the application info
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Framework Configuration
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(config.prefix()))
        // Help
        .help(&HELP)
        // Groups
        .group(&USER_GROUP)
        .group(&STAFF_GROUP)
        .group(&MISC_GROUP)
        .group(&DEV_GROUP)
        // Hooks
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        // Buckets
        .bucket("general", |b| {
            // Bucket for general commands
            b.limit(3)
                .time_span(6)
                .limit_for(LimitedFor::User)
                .delay_action(delay_action)
        })
        .await;

    // Gateway Intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Client Configuration
    let mut client = Client::builder(config.token(), intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    // Shard Manager
    let shard_manager = client.shard_manager.clone();

    // Start all shards
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
