use crate::Achievement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Update {
    pub achievement: Achievement,
}
