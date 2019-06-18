use crate::{sys, InputModeKind};

#[derive(Clone, Copy, Eq, PartialEq, derive_more::From, derive_more::Into)]
pub struct InputMode(pub(crate) sys::DiscordInputMode);

impl InputMode {
    pub fn kind(&self) -> InputModeKind {
        self.0.type_.into()
    }

    get_str!(shortcut, shortcut);

    pub fn empty() -> Self {
        Self(Default::default())
    }

    pub fn with_kind(&'_ mut self, kind: InputModeKind) -> &'_ mut Self {
        self.0.type_ = kind.into();
        self
    }

    set_str!(with_shortcut, shortcut);
}

impl std::fmt::Debug for InputMode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("InputMode")
            .field("kind", &self.kind())
            .field("shortcut", &self.shortcut())
            .finish()
    }
}
