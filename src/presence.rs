use crate::activity::Activity;
use crate::prelude::*;

pub struct Presence {
    pub status: Status,
    pub activity: Activity,
}

pub enum Status {
    DoNotDisturb,
    Idle,
    Offline,
    Online,
}
