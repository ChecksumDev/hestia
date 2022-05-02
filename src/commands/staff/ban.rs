use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

#[command]
#[description("Kicks a user from the server.")]
#[required_permissions("BAN_MEMBERS")]
#[usage("<user> [delete message days] (reason)")]
#[example("@user#1234 7 spamming")]
#[bucket("moderation")]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = match args.single::<UserId>() {
        Ok(user) => user,
        Err(_) => {
            info!("No user specified.");
            info!("{:?}", args);
            msg.channel_id
                .say(&ctx, "Please mention a user to ban.")
                .await?;
            return Ok(());
        }
    };

    let days = args.single::<u8>().unwrap_or(0);
    let user_full = ctx.http.get_user(user.0).await?;

    if user_full.id == msg.author.id {
        msg.reply(&ctx, "Why would you want to ban yourself?")
            .await?;
        return Ok(());
    }

    if user_full.id == ctx.cache.current_user().id {
        msg.reply(&ctx, "I'm sorry, Dave. I'm afraid I can't do that.")
            .await?;
        return Ok(());
    }

    let reason = args.rest();
    if reason.is_empty() {
        msg.reply(&ctx, "Please provide a reason for the ban.")
            .await?;

        return Ok(());
    }

    let guild = ctx.http.get_guild(msg.guild_id.unwrap().0).await?; // This cannot fail since we are checking for a guild in groups.rs;

    // Check if the user is in the guild.
    let member = match guild.member(ctx, user).await {
        Ok(member) => member,
        Err(_) => {
            msg.react(&ctx, '\u{1f44b}').await?;
            msg.reply(&ctx, "That user is not in this guild.").await?;
            return Ok(());
        }
    };

    match member.ban_with_reason(ctx, days, reason).await {
        Ok(_) => {
            msg.react(&ctx, '\u{1f44b}').await?;

            msg.reply(
                ctx,
                &format!(
                    ":boot: **{}** has been banned.\nReason: *{}*",
                    user_full.name, reason
                ),
            )
            .await?;
        }
        Err(_) => {
            msg.reply(
                ctx,
                &format!(
                    ":x: I was unable to ban **{}**.\nPlease make sure I have the correct permissions and that the user is not a higher role than me.",
                    user_full.name
                ),
            )
            .await?;
        }
    }

    Ok(())
}
