use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        event::ResumedEvent,
        gateway::{Activity, Ready},
        user::OnlineStatus,
    },
};
use tracing::log::info;

use crate::Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_presence(
            Some(Activity::streaming(
                "Hestia is bestia!",
                "https://twitch.tv/checksum__",
            )),
            OnlineStatus::Idle,
        )
        .await;
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
