use crate::sys;

bitflags::bitflags! {
    /// User Flags
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/users#data-models-userflag-enum>
    pub struct UserFlags: sys::EDiscordUserFlag {
        const PARTNER = sys::DiscordUserFlag_Partner;
        const HYPE_SQUAD_EVENTS = sys::DiscordUserFlag_HypeSquadEvents;
        const HYPE_SQUAD_HOUSE_1 = sys::DiscordUserFlag_HypeSquadHouse1;
        const HYPE_SQUAD_HOUSE_2 = sys::DiscordUserFlag_HypeSquadHouse2;
        const HYPE_SQUAD_HOUSE_3 = sys::DiscordUserFlag_HypeSquadHouse3;
    }
}
