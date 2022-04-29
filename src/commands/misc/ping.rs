use chrono::Utc;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Checks the bot's latency.")]
#[bucket("general")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Utc::now();

    let mut message = msg.reply_ping(&ctx.http, "Pong!").await?;

    let end = Utc::now();

    let diff = end.signed_duration_since(start).num_milliseconds();

    message
        .edit(&ctx, |m| m.content(format!("Pong! ({}ms)", diff)))
        .await?;

    Ok(())
}
