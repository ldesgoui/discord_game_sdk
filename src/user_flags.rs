use crate::sys;

bitflags::bitflags! {
    /// User Flags
    ///
    /// > [Bitfield in official docs](https://discordapp.com/developers/docs/game-sdk/users#data-models-userflag-enum)
    pub struct UserFlags: sys::EDiscordUserFlag {
        /// Discord Partner
        const PARTNER = sys::DiscordUserFlag_Partner;
        /// HypeSquad Events participant
        const HYPE_SQUAD_EVENTS = sys::DiscordUserFlag_HypeSquadEvents;
        /// House Bravery
        const HYPE_SQUAD_HOUSE_1 = sys::DiscordUserFlag_HypeSquadHouse1;
        /// House Brilliance
        const HYPE_SQUAD_HOUSE_2 = sys::DiscordUserFlag_HypeSquadHouse2;
        /// House Balance
        const HYPE_SQUAD_HOUSE_3 = sys::DiscordUserFlag_HypeSquadHouse3;
    }
}
