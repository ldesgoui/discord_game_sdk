use crate::sys;

/// Activity Join Request Reply
///
/// > [Enum in official docs](https://discordapp.com/developers/docs/game-sdk/activities#data-models-activityjoinrequestreply-enum)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RequestReply {
    /// Accept the request
    Yes,
    /// Deny the request
    No,
    /// Ignore the request
    Ignore,
    /// Safety net for missing definitions
    Undefined(sys::EDiscordActivityJoinRequestReply),
}

impl Into<sys::EDiscordActivityJoinRequestReply> for RequestReply {
    fn into(self) -> sys::EDiscordActivityJoinRequestReply {
        match self {
            Self::Yes => sys::DiscordActivityJoinRequestReply_Yes,
            Self::No => sys::DiscordActivityJoinRequestReply_No,
            Self::Ignore => sys::DiscordActivityJoinRequestReply_Ignore,
            Self::Undefined(n) => n,
        }
    }
}
