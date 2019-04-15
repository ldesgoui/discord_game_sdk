use crate::sys;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RequestReply {
    Yes,
    No,
    Ignore,
}

#[doc(hidden)]
impl Into<sys::EDiscordActivityJoinRequestReply> for RequestReply {
    fn into(self) -> sys::EDiscordActivityJoinRequestReply {
        match self {
            RequestReply::Yes => sys::DiscordActivityJoinRequestReply_Yes,
            RequestReply::No => sys::DiscordActivityJoinRequestReply_No,
            RequestReply::Ignore => sys::DiscordActivityJoinRequestReply_Ignore,
        }
    }
}
