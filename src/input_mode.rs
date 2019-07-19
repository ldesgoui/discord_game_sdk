use crate::{sys, InputModeKind};

/// Input Mode
///
/// <https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-inputmode-struct>
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct InputMode(pub(crate) sys::DiscordInputMode);

impl InputMode {
    pub fn kind(&self) -> InputModeKind {
        self.0.type_.into()
    }

    get_str!(
        "<https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-shortcut-keys>",
        shortcut,
        shortcut
    );

    /// Create a new, empty Input Mode
    pub fn empty() -> Self {
        Self(Default::default())
    }

    pub fn with_kind(&'_ mut self, kind: InputModeKind) -> &'_ mut Self {
        self.0.type_ = kind.into();
        self
    }

    set_str!(
        "<https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-shortcut-keys>",
        with_shortcut,
        shortcut
    );
}

impl std::fmt::Debug for InputMode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("InputMode")
            .field("kind", &self.kind())
            .field("shortcut", &self.shortcut())
            .finish()
    }
}
