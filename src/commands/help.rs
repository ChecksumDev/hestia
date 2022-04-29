use std::collections::HashSet;

use serenity::{
    client::Context,
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};

#[help]
#[individual_command_tip = ":wave: こんにちは！\nIf you want more information about a specific command, just pass the command as argument.\n"]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(8)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
#[embed_success_colour("#BD68F7")]
#[embed_error_colour("#F7766F")]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
