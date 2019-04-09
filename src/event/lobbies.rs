#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Update {
    pub id: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delete {
    pub id: i64,
    pub reason: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberConnect {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberUpdate {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberDisconnect {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub buffer: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Speaking {
    pub id: i64,
    pub user_id: i64,
    pub speaking: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkMessage {
    pub id: i64,
    pub user_id: i64,
    pub chan_id: u8,
    pub buffer: Vec<u8>,
}
