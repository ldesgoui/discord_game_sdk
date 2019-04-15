#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Reliability {
    Reliable,
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
