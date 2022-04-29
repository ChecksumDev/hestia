use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult, Delimiter};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

#[command]
#[description("Unbans a user from the server.")]
#[required_permissions("BAN_MEMBERS")]
#[usage("<user>")]
#[example("@user#1234")]
#[bucket("moderation")]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    args.advance();

    let user = match args.single::<UserId>() {
        Ok(user) => user,
        Err(_) => {
            info!("No user specified.");
            info!("{:?}", args);
            msg.channel_id
                .say(&ctx, "Please mention a user to unban.")
                .await?;
            return Ok(());
        }
    };

    let user_full = ctx.http.get_user(user.0).await?;
    let guild = ctx.http.get_guild(msg.guild_id.unwrap().0).await?;

    if user_full.id == msg.author.id {
        msg.channel_id.say(&ctx, "You're not banned?").await?;
        return Ok(());
    }

    let bans = guild.bans(&ctx).await?;
    let mut banned = false;
    for ban in bans {
        if ban.user.id == user_full.id {
            banned = true;
            break;
        }
    }

    if !banned {
        msg.channel_id.say(&ctx, "That user is not banned.").await?;
        return Ok(());
    }

    match guild.unban(ctx, user).await {
        Ok(_) => {
            msg.react(&ctx, '\u{1f44b}').await?;
            msg.reply(
                &ctx,
                format!("Successfully unbanned {} from the server.", user_full.name),
            )
            .await?;
        }
        Err(why) => {
            msg.react(&ctx, '\u{1f44e}').await?;
            msg.reply(&ctx, format!(":x: I was unable to unban **{}**.\nPlease ensure I have the correct permissions.\n\n```{:?}```", user_full.name, why)).await?;
        }
    }

    Ok(())
}
