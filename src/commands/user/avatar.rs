use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Get the avatar of a user.")]
#[usage("<user>")]
#[example("@Hestia")]
#[bucket("general")]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let user = match msg.mentions.first() {
        Some(user) => user,
        None => &msg.author,
    };

    let avatar = user.avatar_url().unwrap();

    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.image(&avatar)
                    .color(0x7289da)
                    .title(format!("{}'s avatar", user.name))
                    .description(format!("[Click here to open]({})", avatar))
            })
        })
        .await?;

    Ok(())
}
