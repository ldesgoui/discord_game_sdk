use crate::prelude::*;

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum RequestReply {
    Yes,
    No,
    Ignore,
}

impl RequestReply {
    pub(crate) fn to_sys(self) -> sys::EDiscordActivityJoinRequestReply {
        match self {
            RequestReply::Yes => sys::DiscordActivityJoinRequestReply_Yes,
            RequestReply::No => sys::DiscordActivityJoinRequestReply_No,
            RequestReply::Ignore => sys::DiscordActivityJoinRequestReply_Ignore,
        }
    }
}
