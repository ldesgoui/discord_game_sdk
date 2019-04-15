use crate::{to_result::ToResult, Discord, DiscordResult, Reliability};
use std::ffi::CStr;

/// # Networking
impl<'a> Discord<'a> {
    pub fn peer_id(&mut self) -> u64 {
        let mut peer_id = 0;

        unsafe { ffi!(self.get_network_manager().get_peer_id(&mut peer_id)) }

        peer_id
    }

    pub fn flush_network(&mut self) -> DiscordResult<()> {
        unsafe { ffi!(self.get_network_manager().flush()) }.to_result()
    }

    pub fn open_peer(&mut self, peer_id: u64, route: impl AsRef<CStr>) -> DiscordResult<()> {
        unsafe {
            ffi!(self
                .get_network_manager()
                .open_peer(peer_id, route.as_ref().as_ptr()))
        }
        .to_result()
    }

    pub fn update_peer(&mut self, peer_id: u64, route: impl AsRef<CStr>) -> DiscordResult<()> {
        unsafe {
            ffi!(self
                .get_network_manager()
                .update_peer(peer_id, route.as_ref().as_ptr()))
        }
        .to_result()
    }

    pub fn close_peer(&mut self, peer_id: u64) -> DiscordResult<()> {
        unsafe { ffi!(self.get_network_manager().close_peer(peer_id)) }.to_result()
    }

    pub fn open_channel(
        &mut self,
        peer_id: u64,
        chan_id: u8,
        reliable: Reliability,
    ) -> DiscordResult<()> {
        unsafe {
            ffi!(self
                .get_network_manager()
                .open_channel(peer_id, chan_id, reliable.into()))
        }
        .to_result()
    }

    pub fn close_channel(&mut self, peer_id: u64, chan_id: u8) -> DiscordResult<()> {
        unsafe { ffi!(self.get_network_manager().close_channel(peer_id, chan_id)) }.to_result()
    }

    pub fn send_message(
        &mut self,
        peer_id: u64,
        chan_id: u8,
        buf: impl AsRef<[u8]>,
    ) -> DiscordResult<()> {
        let buf = buf.as_ref();

        assert!(buf.len() <= u32::max_value() as usize);

        unsafe {
            ffi!(self.get_network_manager().send_message(
                peer_id,
                chan_id,
                buf.as_ptr() as *mut _,
                buf.len() as u32
            ))
        }
        .to_result()
    }
}
