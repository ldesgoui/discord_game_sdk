use crate::sys;

/// Activity Join Request Reply
///
/// <https://discordapp.com/developers/docs/game-sdk/activities#data-models-activityjoinrequestreply-enum>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RequestReply {
    Yes,
    No,
    Ignore,
    Undefined(sys::EDiscordActivityJoinRequestReply),
}

#[doc(hidden)]
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
