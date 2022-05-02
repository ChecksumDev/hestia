use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Flips a coin.")]
#[aliases("coin", "flip", "flipcoin")]
#[bucket("fun")]
async fn coinflip(ctx: &Context, msg: &Message) -> CommandResult {
    let rng = rand::random();
    let result = if rng { "Heads" } else { "Tails" };

    msg.reply(&ctx, format!("The coin landed on {}.", result))
        .await?;
    Ok(())
}
