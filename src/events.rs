use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        event::ResumedEvent,
        gateway::{Activity, Ready},
        guild::{Guild, UnavailableGuild},
        user::OnlineStatus,
    },
};
use tracing::log::info;

use crate::{utils::config::Config, Handler};

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let config = Config::get_config("config.toml");

        ctx.set_presence(
            Some(Activity::streaming(
                format!(
                    "{}help | Watching over {} guilds",
                    config.prefix(),
                    ready.guilds.len()
                ),
                "https://twitch.tv/checksum__",
            )),
            OnlineStatus::Idle,
        )
        .await;
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _ctx: Context, _resumed: ResumedEvent) {
        info!("Resumed");
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: bool) {
        if is_new {
            info!("Joined guild {}", guild.name);
        }
    }

    async fn guild_delete(&self, ctx: Context, guild: UnavailableGuild, _full: Option<Guild>) {
        let guild = ctx.http.get_guild(guild.id.0).await;
        info!("Left guild {}", guild.unwrap().name);
    }
}
