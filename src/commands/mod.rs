pub mod admin {
    // #[path("admin.rs")]
}

pub mod staff {
    // TODO: Kick, ban, unban, mute, unmute, warn, unwarn, warnlist, etc
    #[path = "kick.rs"]
    pub mod kick;

    #[path = "ban.rs"]
    pub mod ban;

    #[path = "unban.rs"]
    pub mod unban;
}

pub mod fun {
    #[path = "coinflip.rs"]
    pub mod coinflip;

    #[path = "8ball.rs"]
    pub mod eightball;

    #[path = "roll.rs"]
    pub mod roll;

    #[path = "dadjoke.rs"]
    pub mod dadjoke;

    // #[path = "slots.rs"]
    // pub mod slots;

    // #[path = "rps.rs"]
    // pub mod rps;

    #[path = "cat.rs"]
    pub mod cat;

    #[path = "dog.rs"]
    pub mod dog;

    #[path = "waifu.rs"]
    pub mod waifu;
}

pub mod user {
    #[path = "avatar.rs"]
    pub mod avatar;

    #[path = "profile.rs"]
    pub mod profile;
    // TODO: Profile, avatar, etc
}

pub mod misc {
    #[path = "ping.rs"]
    pub mod ping;
}

pub mod help;

pub mod dev {
    #[path = "shutdown.rs"]
    pub mod shutdown;
}
