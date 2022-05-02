use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;


#[command]
#[description("Kicks a user from the server.")]
#[required_permissions("KICK_MEMBERS")]
#[usage("<user> [reason]")]
#[example("@user#1234 spamming")]
#[bucket("moderation")]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = match args.single::<UserId>() {
        Ok(user) => user,
        Err(_) => {
            msg.reply(&ctx, "Please mention a user to kick.").await?;
            return Ok(());
        }
    };

    let user_full = ctx.http.get_user(user.0).await?;

    if user_full.id == msg.author.id {
        msg.reply(&ctx, "Why would you want to kick yourself?")
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
        msg.reply(&ctx, "Please provide a reason for the kick.")
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

    match member.kick_with_reason(ctx, reason).await {
        Ok(_) => {
            msg.react(&ctx, '\u{1f44b}').await?;

            msg.reply(
                ctx,
                &format!(
                    ":boot: **{}** has been kicked.\nReason: ***`{}`***",
                    user_full.name, reason
                ),
            )
            .await?;
        }
        Err(_) => {
            msg.reply(
                ctx,
                &format!(
                    ":x: I was unable to kick **{}**.\nPlease make sure I have the correct permissions and that the user is not a higher role than me.",
                    user_full.name
                ),
            )
            .await?;
        }
    }

    Ok(())
}
