

use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Rolls a dice of a given size.")]
#[aliases("dice")]
#[bucket("fun")]
async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let size = args.single::<u32>().unwrap_or(10);

    if size <= 1 {
        msg.reply(&ctx, "The dice must have a size of at least 2.").await?;
        return Ok(());
    }

    let mut rng = StdRng::from_entropy();
    let result = rng.gen_range(1..size);

    msg.reply(&ctx, format!("You rolled {}! (1-{})", result, size))
        .await?;

    Ok(())
}
