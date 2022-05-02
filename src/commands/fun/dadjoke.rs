use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("Gets a dad joke.")]
#[aliases("dad", "dj")]
#[bucket("fun")]
async fn dadjoke(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let url = "https://icanhazdadjoke.com/";

    // Send a GET request to the URL with the header "Accept" set to "plain/text"
    let res = client
        .get(url)
        .header("Accept", "text/plain")
        .send()
        .await?;

    // Check if the response is successful
    if !res.status().is_success() {
        msg.reply(&ctx, "Failed to get a dad joke, the end is nigh!")
            .await?;
        return Ok(());
    }

    // Read the response body
    let body = res.text().await?;

    // Send the response body to the channel
    msg.reply(&ctx, &body).await?;

    Ok(())
}
