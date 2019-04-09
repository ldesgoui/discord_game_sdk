#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub peer_id: u64,
    pub chan_id: u8,
    pub buffer: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteUpdate {
    pub route: String,
}
