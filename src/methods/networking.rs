use crate::{event, to_result::ToResult, Discord, Reliability, Result};
use std::borrow::Cow;

/// # Networking
///
/// Lower level networking functionality.
///
/// <https://discordapp.com/developers/docs/game-sdk/networking>
impl<'a> Discord<'a> {
    /// Get the networking peer ID for the current user, allowing other users to send packets to them.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#getpeerid>
    pub fn peer_id(&self) -> u64 {
        let mut peer_id = 0;

        unsafe { ffi!(self.get_network_manager().get_peer_id(&mut peer_id)) }

        peer_id
    }

    /// Flushes the network. Run this near the end of your game's loop,
    /// once you've finished sending all you need to send.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#flush>
    pub fn flush_network(&self) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().flush()) }.to_result()
    }

    /// Opens a network connection to another Discord user.
    ///
    /// A nul byte will be appended to `route` if necessary.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#openpeer>
    pub fn open_peer<'b>(&self, peer_id: u64, route: impl Into<Cow<'b, str>>) -> Result<()> {
        let mut route = route.into();

        if !route.contains('\0') {
            route.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_network_manager()
                .open_peer(peer_id, route.as_ptr() as *const _))
        }
        .to_result()
    }

    /// Updates the network connection to another Discord user.
    /// You'll want to call this when notified that the route to another user has changed,
    /// most likely from a lobby member update event.
    ///
    /// A nul byte will be appended to `route` if necessary.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#updatepeer>
    pub fn update_peer<'b>(&self, peer_id: u64, route: impl Into<Cow<'b, str>>) -> Result<()> {
        let mut route = route.into();

        if !route.contains('\0') {
            route.to_mut().push('\0')
        };

        unsafe {
            ffi!(self
                .get_network_manager()
                .update_peer(peer_id, route.as_ptr() as *const _))
        }
        .to_result()
    }

    /// Disconnects the network session to another Discord user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#closepeer>
    pub fn close_peer(&self, peer_id: u64) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().close_peer(peer_id)) }.to_result()
    }

    /// Opens a network connection to another Discord user.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#openchannel>
    pub fn open_channel(&self, peer_id: u64, chan_id: u8, reliable: Reliability) -> Result<()> {
        unsafe {
            ffi!(self
                .get_network_manager()
                .open_channel(peer_id, chan_id, reliable.into()))
        }
        .to_result()
    }

    /// Close the connection to a given user by peer ID on the given channel.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#closechannel>
    pub fn close_channel(&self, peer_id: u64, chan_id: u8) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().close_channel(peer_id, chan_id)) }.to_result()
    }

    /// Sends data to a given peer ID through the given channel.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#sendmessage>
    pub fn send_message(&self, peer_id: u64, chan_id: u8, buf: impl AsRef<[u8]>) -> Result<()> {
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

    /// Fires when you receive data from another user.
    /// This callback will only fire if you already have an open channel with the user sending you data.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#onmessage>
    pub fn recv_networking_message(&self) -> impl '_ + Iterator<Item = event::networking::Message> {
        self.receivers.networking_message.try_iter()
    }

    /// Fires when your networking route has changed. You should broadcast this change to other users.
    ///
    /// <https://discordapp.com/developers/docs/game-sdk/networking#onrouteupdate>
    pub fn recv_networking_route_update(
        &self,
    ) -> impl '_ + Iterator<Item = event::networking::RouteUpdate> {
        self.receivers.networking_route_update.try_iter()
    }
}
