use crate::sys;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Distance {
    Default,
    Extended,
    Global,
    Local,
}

impl Into<sys::EDiscordLobbySearchDistance> for Distance {
    fn into(self) -> sys::EDiscordLobbySearchDistance {
        match self {
            Distance::Default => sys::DiscordLobbySearchDistance_Default,
            Distance::Extended => sys::DiscordLobbySearchDistance_Extended,
            Distance::Global => sys::DiscordLobbySearchDistance_Global,
            Distance::Local => sys::DiscordLobbySearchDistance_Local,
        }
    }
}
