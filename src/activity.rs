use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Activity {
    pub kind: ActivityKind,
    pub application_id: i64,
    pub name: String,
    pub state: String,
    pub details: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
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

impl FromSys for Activity {
    type Source = sys::DiscordActivity;

    fn from_sys(source: &Self::Source) -> Self {
        use chrono::offset::TimeZone;

        unsafe {
            Self {
                kind: ActivityKind::from_sys(&source.type_),
                application_id: source.application_id,
                name: string_from_cstr(&source.name as *const _),
                state: string_from_cstr(&source.state as *const _),
                details: string_from_cstr(&source.state as *const _),
                start_time: chrono::Utc.timestamp(source.timestamps.start, 0),
                end_time: chrono::Utc.timestamp(source.timestamps.end, 0),
                large_image_key: string_from_cstr(&source.assets.large_image as *const _),
                large_image_tooltip: string_from_cstr(&source.assets.large_text as *const _),
                small_image_key: string_from_cstr(&source.assets.small_image as *const _),
                small_image_tooltip: string_from_cstr(&source.assets.small_text as *const _),
                party_id: string_from_cstr(&source.party.id as *const _),
                party_amount: source.party.size.current_size,
                party_capacity: source.party.size.max_size,
                instance: source.instance,
                match_secret: string_from_cstr(&source.secrets.match_ as *const _),
                join_secret: string_from_cstr(&source.secrets.join as *const _),
                spectate_secret: string_from_cstr(&source.secrets.spectate as *const _),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivityKind {
    Listening,
    Playing,
    Streaming,
    Watching,
}

impl FromSys for ActivityKind {
    type Source = sys::EDiscordActivityType;

    fn from_sys(source: &Self::Source) -> Self {
        match *source {
            sys::DiscordActivityType_Listening => ActivityKind::Listening,
            sys::DiscordActivityType_Playing => ActivityKind::Playing,
            sys::DiscordActivityType_Streaming => ActivityKind::Streaming,
            sys::DiscordActivityType_Watching => ActivityKind::Watching,
            _ => panic!("enum"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Join,
    Spectate,
}

impl Action {
    pub(crate) fn to_sys(self) -> sys::EDiscordActivityActionType {
        match self {
            Action::Join => sys::DiscordActivityActionType_Join,
            Action::Spectate => sys::DiscordActivityActionType_Spectate,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestReply {
    Yes,
    No,
    Ignore,
}

impl RequestReply {
    pub(crate) fn to_sys(self) -> sys::EDiscordActivityJoinRequestReply {
        match self {
            RequestReply::Yes => sys::DiscordActivityJoinRequestReply_Yes,
            RequestReply::No => sys::DiscordActivityJoinRequestReply_No,
            RequestReply::Ignore => sys::DiscordActivityJoinRequestReply_Ignore,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ActivityChange<'a> {
    pub state: Option<&'a str>,
    pub details: Option<&'a str>,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub large_image_key: Option<&'a str>,
    pub large_image_tooltip: Option<&'a str>,
    pub small_image_key: Option<&'a str>,
    pub small_image_tooltip: Option<&'a str>,
    pub party_id: Option<&'a str>,
    pub party_amount: Option<i32>,
    pub party_capacity: Option<i32>,
    pub match_secret: Option<&'a str>,
    pub join_secret: Option<&'a str>,
    pub spectate_secret: Option<&'a str>,
}

impl<'a> ActivityChange<'a> {
    pub(crate) fn to_sys(&self) -> sys::DiscordActivity {
        let mut activity = sys::DiscordActivity::default();

        write_to_array(&self.state, &mut activity.state);
        write_to_array(&self.details, &mut activity.details);
        write_to_array(&self.large_image_key, &mut activity.assets.large_image);
        write_to_array(&self.large_image_tooltip, &mut activity.assets.large_text);
        write_to_array(&self.small_image_key, &mut activity.assets.small_image);
        write_to_array(&self.small_image_tooltip, &mut activity.assets.small_text);
        write_to_array(&self.party_id, &mut activity.party.id);
        write_to_array(&self.match_secret, &mut activity.secrets.match_);
        write_to_array(&self.join_secret, &mut activity.secrets.join);
        write_to_array(&self.spectate_secret, &mut activity.secrets.spectate);

        if let Some(start_time) = self.start_time {
            activity.timestamps.start = start_time.timestamp();
        }

        if let Some(end_time) = self.end_time {
            activity.timestamps.end = end_time.timestamp();
        }

        if let Some(party_amount) = self.party_amount {
            activity.party.size.current_size = party_amount;
        }

        if let Some(party_capacity) = self.party_capacity {
            activity.party.size.max_size = party_capacity;
        }

        activity
    }
}

fn write_to_array(source: &Option<&str>, destination: &mut [i8]) {
    if let Some(src) = *source {
        let cstring = std::ffi::CString::new(src).unwrap();

        let bytes: &[i8] =
            unsafe { (cstring.as_bytes_with_nul() as *const [u8] as *const [i8]).as_ref() }
                .unwrap();

        if bytes.len() > destination.len() {
            panic!("2large");
        }

        destination[..bytes.len()].copy_from_slice(bytes);
    }
}
