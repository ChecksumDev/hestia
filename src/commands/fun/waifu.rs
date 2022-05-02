use std::fmt::{Debug};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

// Derive deserialize for the struct
#[derive(Debug, Deserialize, Serialize)]
struct Waifu {
    url: String,
}

#[command]
#[description("Returns a image/gif of a waifu based on the user's input")]
#[aliases("w")]
#[bucket("fun")]
pub async fn waifu(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let choices = vec![
        "bully", "cuddle", "cry", "hug", "awoo", "kiss", "lick", "pat", "smug", "bonk", "yeet",
        "blush", "smile", "wave", "highfive", "handhold", "nom", "bite", "glomp", "slap", "kill",
        "kick", "happy", "wink", "poke", "dance", "cringe",
    ];

    let choice_responses = vec![
        "{author} bullies {user}!",
        "{author} cuddles {user}!",
        "{author} cries on {user}'s shoulder!",
        "{author} hugs {user}!",
        "{author} awoo's at {user}!",
        "{author} kisses {user}!",
        "{author} licks {user}!",
        "{author} pats {user}!",
        "{author} is smug at {user}!",
        "{author} bonks {user}!",
        "{author} yeets {user}!",
        "{author} blushes at {user}!",
        "{author} smiles at {user}!",
        "{author} waves at {user}!",
        "{author} highfives {user}!",
        "{author} holds {user}'s hand!",
        "{author} nom's {user}!",
        "{author} bites {user}!",
        "{author} glomps {user}!",
        "{author} slaps {user}!",
        "{author} kills {user}!",
        "{author} kicks {user}!",
        "{author} is happy with {user}!",
        "{author} winks at {user}!",
        "{author} pokes {user}!",
        "{author} dances around {user}!",
        "{author} cringes at {user}!",
    ];

    let waifu_type = match args.single::<String>() {
        Ok(waifu_type) => waifu_type,
        Err(_) => {
            msg.reply(
                &ctx,
                format!(
                    "Please specify a valid argument.\nAvailable types: *{}*",
                    choices.join(", ")
                ),
            )
            .await?;
            return Ok(());
        }
    };

    let user = args
        .single::<UserId>()
        .unwrap_or_else(|_| ctx.cache.current_user().id);
    let user_full = ctx.http.get_user(user.0).await?;

    let waifu_type = waifu_type.to_lowercase();
    if !choices.contains(&waifu_type.as_str()) {
        msg.channel_id
            .say(
                &ctx,
                format!("Please specify a waifu type.\n`{}", choices.join("`, ")),
            )
            .await?;
        return Ok(());
    }

    let client = reqwest::Client::new();
    let url = "https://api.waifu.pics/sfw/".to_owned() + &waifu_type;
    let res = client.get(&url).send().await?;

    if !res.status().is_success() {
        msg.channel_id
            .say(&ctx, format!("Failed to get a waifu, the end is nigh!"))
            .await?;
        return Ok(());
    }

    let body: Waifu = res.json().await?;
    let response =
        choice_responses[choices.iter().position(|&x| x == waifu_type).unwrap()].to_string();

    let response = response
        .replace("{author}", &msg.author.name)
        .replace("{user}", &user_full.name);

    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(response.clone());
                    a.url(body.url.as_str());
                    a.icon_url(msg.author.face());
                    a
                });
                e.image(body.url.as_str());
                e.color(0xBD68F7)
                    .footer(|f| f.text("Powered by Waifu.pics"))
                    .timestamp(Utc::now().to_rfc3339());
                e
            });
            m
        })
        .await?;

    Ok(())
}
