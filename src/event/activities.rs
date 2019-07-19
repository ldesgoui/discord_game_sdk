use crate::{Action, Activity, User};

/// On Activity Join
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoin>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Join {
    pub secret: String,
}

/// On Activity Spectate
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityspectate>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Spectate {
    pub secret: String,
}

/// On Activity Join Request
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityjoinrequest>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Request {
    pub user: User,
}

/// On Activity Invitation
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#onactivityinvite>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Invite {
    pub action: Action,
    pub user: User,
    pub activity: Activity,
}
