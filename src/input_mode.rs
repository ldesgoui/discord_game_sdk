use crate::{
    sys,
    utils::{charbuf_to_str, write_charbuf},
    InputModeKind,
};

/// Input Mode
///
/// > [Struct in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-inputmode-struct)  
/// > [Shortcut keys in official docs](https://discordapp.com/developers/docs/game-sdk/discord-voice#data-models-shortcut-keys)
#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
#[repr(transparent)]
pub struct InputMode(pub(crate) sys::DiscordInputMode);

impl InputMode {
    /// What triggers voice to be transmitted
    pub fn kind(&self) -> InputModeKind {
        self.0.type_.into()
    }

    /// The combination of keys to transmit voice when kind is [`PushToTalk`].
    ///
    /// [`PushToTalk`]: enum.InputModeKind.html#variant.PushToTalk
    pub fn shortcut(&self) -> &str {
        charbuf_to_str(&self.0.shortcut)
    }

    /// Create a new Input Mode with kind [`VoiceActivity`].
    ///
    /// [`VoiceActivity`]: enum.InputModeKind.html#variant.VoiceActivity
    pub fn voice_activity() -> Self {
        Self(sys::DiscordInputMode {
            type_: sys::DiscordInputModeType_VoiceActivity,
            ..sys::DiscordInputMode::default()
        })
    }

    /// Create a new Input Mode with kind [`PushToTalk`] and a shortcut.
    ///
    /// Only the first 256 bytes will be written.
    ///
    /// [`PushToTalk`]: enum.InputModeKind.html#variant.PushToTalk
    pub fn push_to_talk(shortcut: &str) -> Self {
        let mut mode = sys::DiscordInputMode {
            type_: sys::DiscordInputModeType_PushToTalk,
            ..sys::DiscordInputMode::default()
        };

        write_charbuf(&mut mode.shortcut, shortcut);

        Self(mode)
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
