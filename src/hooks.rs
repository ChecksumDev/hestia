use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult, DispatchError},
    model::channel::Message,
};
use tracing::{debug, error, info, warn};

#[hook]
pub async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true
}

#[hook]
pub async fn after(
    _ctx: &Context,
    _msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    match command_result {
        Ok(()) => debug!("Processed command '{}'", command_name),
        Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    warn!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
pub async fn normal_message(_ctx: &Context, _msg: &Message) {
    // info!("[{}] {}", msg.channel_id, msg.content);
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    let msg = msg.react(ctx, '⏱').await;
    if let Err(why) = msg {
        error!("Failure adding reaction to delay message: {:?}", why);
    }
}

#[hook]
pub async fn dispatch_error(
    ctx: &Context,
    msg: &Message,
    error: DispatchError,
    _command_name: &str,
) {
    if let DispatchError::Ratelimited(info) = error {
        if info.is_first_try {
            let _ = msg
                .reply(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", info.as_secs()),
                )
                .await;
        }
    }
}
