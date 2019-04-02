use crate::prelude::*;

pub struct Activity {
    pub kind: ActivityKind,
    pub application_id: i64,
    pub name: String,
    pub state: String,
    pub details: String,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub large_image_key: String,
    pub large_image_tooltip: String,
    pub small_image_key: String,
    pub small_image_tooltip: String,
    pub party_id: String,
    pub party_amount: i32,
    pub party_capacity: i32,
    pub instance: bool,
    pub match_secret: String,
    pub join_secret: String,
    pub spectate_secret: String,
}

pub enum ActivityKind {
    Listening,
    Playing,
    Streaming,
    Watching,
}
