/// Network Channel Reliability
///
/// <https://discordapp.com/developers/docs/game-sdk/networking>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Reliability {
    /// All data will be received
    Reliable,
    /// Some data will be lost
    Unreliable,
}

impl Into<bool> for Reliability {
    fn into(self) -> bool {
        match self {
            Reliability::Reliable => true,
            Reliability::Unreliable => false,
        }
    }
}
