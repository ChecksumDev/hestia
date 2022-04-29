use crate::commands::dev::shutdown::*;
use crate::commands::misc::ping::*;
use crate::commands::staff::{ban::*, kick::*, unban::*};
use crate::commands::user::{avatar::*};
use serenity::framework::standard::macros::group;

#[group]
#[commands(avatar)]
struct User;

#[group]
#[commands(ping)]
struct Misc;

#[group]
#[only_in("guilds")]
#[commands(kick, ban, unban)]
struct Staff;

#[group]
#[commands(shutdown)]
struct Dev;
