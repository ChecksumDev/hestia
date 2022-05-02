pub mod buckets;
mod commands;
mod events;
pub mod groups;
pub mod hooks;
mod utils;

use std::collections::HashSet;
use std::sync::Arc;

use buckets::general_bucket;
use commands::help::HELP;
use groups::{DEV_GROUP, FUN_GROUP, MISC_GROUP, STAFF_GROUP, USER_GROUP};
use mongodb::{options::ClientOptions as MongoClientOptions, Client as MongoClient};

use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::http::Http;

use serenity::prelude::*;
use tracing::{error, info};

use hooks::{after, before, dispatch_error, normal_message, unknown_command};
use utils::config::Config;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Database;

impl TypeMapKey for Database {
    type Value = Arc<Mutex<MongoClient>>;
}

struct Handler; // For use in events/

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // Initialize tracing

    let config = Config::get_config("config.toml");
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
        .group(&FUN_GROUP)
        .group(&MISC_GROUP)
        .group(&DEV_GROUP)
        // Hooks
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        // Buckets
        .bucket("general", add_bucket!(general_bucket))
        .await;

    // Gateway Intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Database
    let client_options = MongoClientOptions::parse(config.mongo_uri()).await.unwrap();

    let mongo_client = match MongoClient::with_options(client_options) {
        Ok(mongo) => {
            info!("Successfully connected to MongoDB");
            mongo
        }
        Err(_) => {
            panic!("Could not connect to MongoDB");
        }
    };

    // Client Configuration
    let mut client = Client::builder(config.token(), intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<Database>(Arc::new(Mutex::new(mongo_client)));
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
