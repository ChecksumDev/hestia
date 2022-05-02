use chrono::Utc;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CatResponse {
    file: String,
}

#[command]
#[description("Gets a random cat image.")]
#[aliases("kitty", "kitten")]
#[bucket("fun")]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let url = "http://aws.random.cat/meow";

    // Send a GET request to the URL requesting the JSON struct "CatResponse"
    let res = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await?;

    // Check if the response is successful
    if !res.status().is_success() {
        msg.reply(
            &ctx,
            format!(
                "Failed to get a cat image, the end is nigh!\n```{}```",
                res.status()
            ),
        )
        .await?;
        return Ok(());
    }

    // Read the response body
    let body = res.json::<CatResponse>().await?;

    // Send the response body to the channel
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.image(body.file.as_str())
                    .title("A cute kitty!")
                    .color(0xFFB6C1)
                    .footer(|f| {
                        f.text(format!(
                            "Requested by {} | Powered by random.cat",
                            msg.author.name
                        ))
                    })
                    .timestamp(Utc::now().to_rfc3339())
            })
        })
        .await?;

    Ok(())
}
