use crate::{to_result::ToResult, Discord, Reliability, Result};

/// # Networking
///
/// <https://discordapp.com/developers/docs/game-sdk/networking>
impl<'a> Discord<'a> {
    /// <https://discordapp.com/developers/docs/game-sdk/networking#getpeerid>
    pub fn peer_id(&mut self) -> u64 {
        let mut peer_id = 0;

        unsafe { ffi!(self.get_network_manager().get_peer_id(&mut peer_id)) }

        peer_id
    }

    /// <https://discordapp.com/developers/docs/game-sdk/networking#flush>
    pub fn flush_network(&mut self) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().flush()) }.to_result()
    }

    /// `route` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#openpeer>
    pub fn open_peer(&mut self, peer_id: u64, mut route: String) -> Result<()> {
        route.push('\0');

        unsafe {
            ffi!(self
                .get_network_manager()
                .open_peer(peer_id, route.as_ptr() as *const _))
        }
        .to_result()
    }

    /// `route` must not contain any nul bytes, it will grow by one byte.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#updatepeer>
    pub fn update_peer(&mut self, peer_id: u64, mut route: String) -> Result<()> {
        route.push('\0');

        unsafe {
            ffi!(self
                .get_network_manager()
                .update_peer(peer_id, route.as_ptr() as *const _))
        }
        .to_result()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/networking#closepeer>
    pub fn close_peer(&mut self, peer_id: u64) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().close_peer(peer_id)) }.to_result()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/networking#openchannel>
    pub fn open_channel(&mut self, peer_id: u64, chan_id: u8, reliable: Reliability) -> Result<()> {
        unsafe {
            ffi!(self
                .get_network_manager()
                .open_channel(peer_id, chan_id, reliable.into()))
        }
        .to_result()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/networking#closechannel>
    pub fn close_channel(&mut self, peer_id: u64, chan_id: u8) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().close_channel(peer_id, chan_id)) }.to_result()
    }

    /// <https://discordapp.com/developers/docs/game-sdk/networking#sendmessage>
    pub fn send_message(&mut self, peer_id: u64, chan_id: u8, buf: impl AsRef<[u8]>) -> Result<()> {
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
