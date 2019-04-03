use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Activity {
    Join {
        secret: String,
    },
    Spectate {
        secret: String,
    },
    Request {
        user: crate::User,
    },
    Invite {
        user: crate::User,
        activity: crate::Activity,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Lobby {
    Update {
        id: i64,
    },
    Delete {
        id: i64,
        reason: u32,
    },
    MemberConnect {
        id: i64,
        user_id: i64,
    },
    MemberUpdate {
        id: i64,
        user_id: i64,
    },
    MemberDisconnect {
        id: i64,
        user_id: i64,
    },
    Message {
        id: i64,
        user_id: i64,
        buffer: Vec<u8>,
    },
    Speaking {
        id: i64,
        user_id: i64,
        speaking: bool,
    },
    NetworkMessage {
        id: i64,
        user_id: i64,
        chan_id: u8,
        buffer: Vec<u8>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Network {
    Message {
        peer_id: u64,
        chan_id: u8,
        buffer: Vec<u8>,
    },
    RouteUpdate {
        route: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Overlay {
    Opened,
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Relationship {
    Refresh,
    Update { relationship: crate::Relationship },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum User {
    CurrentUserUpdated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Voice {
    SettingsUpdated,
}
