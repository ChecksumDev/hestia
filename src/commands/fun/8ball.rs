use rand::prelude::{SliceRandom, StdRng};
use rand::{SeedableRng};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Rolls an eight-ball.")]
#[aliases("8ball", "8b")]
#[bucket("fun")]
async fn eightball(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(&ctx, "You need to ask a question!").await?;
        return Ok(());
    }

    let responses = vec![
        "It is certain.",
        "It is decidedly so.",
        "Without a doubt.",
        "Yes - definitely.",
        "You may rely on it.",
        "As I see it, yes.",
        "Most likely.",
        "Outlook good.",
        "Yes.",
        "Signs point to yes.",
        "Reply hazy, try again.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don't count on it.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Very doubtful.",
    ];

    let response = responses.choose(&mut StdRng::from_entropy()).unwrap();
    msg.reply(&ctx, response).await?;
    Ok(())
}
