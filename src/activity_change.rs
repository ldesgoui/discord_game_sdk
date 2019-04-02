use crate::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
    pub(crate) fn to_sys(&self) -> Result<sys::DiscordActivity> {
        let mut activity = sys::DiscordActivity::default();

        write_to_array(&self.state, &mut activity.state)?;
        write_to_array(&self.details, &mut activity.details)?;
        write_to_array(&self.large_image_key, &mut activity.assets.large_image)?;
        write_to_array(&self.large_image_tooltip, &mut activity.assets.large_text)?;
        write_to_array(&self.small_image_key, &mut activity.assets.small_image)?;
        write_to_array(&self.small_image_tooltip, &mut activity.assets.small_text)?;
        write_to_array(&self.party_id, &mut activity.party.id)?;
        write_to_array(&self.match_secret, &mut activity.secrets.match_)?;
        write_to_array(&self.join_secret, &mut activity.secrets.join)?;
        write_to_array(&self.spectate_secret, &mut activity.secrets.spectate)?;

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

        Ok(activity)
    }
}

fn write_to_array(source: &Option<&str>, destination: &mut [i8]) -> Result<()> {
    if let &Some(src) = source {
        let cstring = std::ffi::CString::new(src).map_err(DeveloperViolation::from)?;
        let bytes: &[i8] = unsafe { std::mem::transmute(cstring.as_bytes_with_nul()) };
        if bytes.len() > destination.len() {
            Err(DeveloperViolation::StringTooLarge)?;
        }
        destination[..bytes.len()].copy_from_slice(bytes);
    }

    Ok(())
}
