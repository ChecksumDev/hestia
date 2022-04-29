pub mod dev {
    #[path = "shutdown.rs"]
    pub mod shutdown;
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
    // TODO: Help, ping, etc
}

pub mod help;
