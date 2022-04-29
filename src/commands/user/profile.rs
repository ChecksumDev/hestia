use chrono::Local;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;

use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Shows the user's profile.")]
#[usage("<user>")]
#[example("@Hestia")]
#[bucket("general")]
async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let user_part = match msg.mentions.first() {
        Some(user) => user,
        None => &msg.author,
    };
    let user = ctx.http.get_user(user_part.id.0).await?;

    let avatar = user.face();
    let accent_color = user.accent_colour.unwrap_or_else(|| 0x7289da.into());

    let account_created =
        chrono::DateTime::parse_from_rfc3339(&user.created_at().to_string()).unwrap();

    let human_readable = timeago::Formatter::new()
        .num_items(4)
        .convert_chrono(account_created, Local::now());

    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(format!("{}'s profile", user.name))
                        .icon_url(ctx.cache.current_user().face())
                })
                .image(
                    user.banner_url().unwrap_or_else(|| {
                        "https://wallpaperaccess.com/full/2930341.jpg".to_string()
                    }),
                )
                .footer(|f| f.text(format!("Account created {}", human_readable)))
                .thumbnail(avatar)
                .colour(accent_color)
            })
        })
        .await?;

    Ok(())
}
