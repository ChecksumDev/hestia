use chrono::Utc;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct DogResponse {
    message: String,
    status: String,
}

#[command]
#[description("Gets a random dog image.")]
#[aliases("puppy")]
#[bucket("fun")]
async fn dog(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let url = "https://dog.ceo/api/breeds/image/random";

    // Send a GET request to the URL requesting the JSON struct "CatResponse"
    let res = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await?;

    // Check if the response is successful
    if !res.status().is_success() {
        msg.reply(&ctx, "Failed to get a dog image, the end is nigh!")
            .await?;
        return Ok(());
    }

    // Read the response body
    let body = res.json::<DogResponse>().await?;

    // Send the response body to the channel
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.image(body.message.as_str())
                    .title("A cute puppy!")
                    .color(0xFFB6C1)
                    .footer(|f| {
                        f.text(format!(
                            "Requested by {} | Powered by dog.ceo",
                            msg.author.name
                        ))
                    })
                    .timestamp(Utc::now().to_rfc3339())
            })
        })
        .await?;

    Ok(())
}
