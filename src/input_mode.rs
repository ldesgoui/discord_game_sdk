use crate::{
    sys,
    utils::{charbuf_to_str, write_charbuf},
    InputModeKind,
};

/// Input Mode
///
/// <https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-inputmode-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct InputMode(pub(crate) sys::DiscordInputMode);

impl InputMode {
    /// What triggers voice to be transmitted
    pub fn kind(&self) -> InputModeKind {
        self.0.type_.into()
    }

    /// The combination of keys to transmit voice when kind is PushToTalk
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-shortcut-keys>
    pub fn shortcut(&self) -> &str {
        charbuf_to_str(&self.0.shortcut)
    }

    /// Create a new, empty Input Mode
    pub fn empty() -> Self {
        Self::from(sys::DiscordInputMode::default())
    }

    /// The combination of keys to transmit voice when kind is PushToTalk
    ///
    /// What triggers the voice to be sent
    pub fn with_kind(&'_ mut self, kind: InputModeKind) -> &'_ mut Self {
        self.0.type_ = kind.into();
        self
    }

    /// The combination of keys to transmit voice when kind is PushToTalk
    ///
    /// `value` *MUST NOT* contain nul bytes
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-shortcut-keys>
    pub fn with_shortcut(&'_ mut self, value: &str) -> &'_ mut Self {
        write_charbuf(&mut self.0.shortcut, value);
        self
    }
}

impl std::fmt::Debug for InputMode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("InputMode")
            .field("kind", &self.kind())
            .field("shortcut", &self.shortcut())
            .finish()
    }
}
