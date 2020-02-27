use crate::{to_result::ToResult, Discord, NetworkChannelID, NetworkPeerID, Reliability, Result};
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
};

/// # Networking
///
/// Lower level networking functionality.
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/networking)
impl<E> Discord<'_, E> {
    /// Get the networking peer ID for the current user, allowing other users to send packets to them.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#getpeerid)
    pub fn peer_id(&self) -> NetworkPeerID {
        let mut peer_id = 0;

        self.with_network_manager(|mgr| unsafe { mgr.get_peer_id.unwrap()(mgr, &mut peer_id) });

        peer_id
    }

    /// Flushes the network. Run this near the end of your game's loop,
    /// once you've finished sending all you need to send.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#flush)
    pub fn flush_network(&self) -> Result<()> {
        self.with_network_manager(|mgr| unsafe { mgr.flush.unwrap()(mgr) })
            .to_result()
    }

    /// Opens a network connection to another Discord user.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `route` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#openpeer)
    pub fn open_peer<'s>(
        &self,
        peer_id: NetworkPeerID,
        route: impl Into<Cow<'s, str>>,
    ) -> Result<()> {
        let mut route = route.into();

        if !route.ends_with('\0') {
            route.to_mut().push('\0')
        };

        self.with_network_manager(|mgr| unsafe {
            mgr.open_peer.unwrap()(mgr, peer_id, route.as_ptr())
        })
        .to_result()
    }

    /// Updates the network connection to another Discord user.
    ///
    /// You'll want to call this when notified that the route to another user has changed,
    /// most likely from a lobby member update event.
    ///
    /// ## Performance
    ///
    /// A nul byte will be appended to `route` if one is not present.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#updatepeer)
    pub fn update_peer<'s>(
        &self,
        peer_id: NetworkPeerID,
        route: impl Into<Cow<'s, str>>,
    ) -> Result<()> {
        let mut route = route.into();

        if !route.ends_with('\0') {
            route.to_mut().push('\0')
        };

        self.with_network_manager(|mgr| unsafe {
            mgr.update_peer.unwrap()(mgr, peer_id, route.as_ptr())
        })
        .to_result()
    }

    /// Disconnects the network session to another Discord user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#closepeer)
    pub fn close_peer(&self, peer_id: NetworkPeerID) -> Result<()> {
        self.with_network_manager(|mgr| unsafe { mgr.close_peer.unwrap()(mgr, peer_id) })
            .to_result()
    }

    /// Opens a network connection to another Discord user.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#openchannel)
    pub fn open_channel(
        &self,
        peer_id: NetworkPeerID,
        channel_id: NetworkChannelID,
        reliable: Reliability,
    ) -> Result<()> {
        self.with_network_manager(|mgr| unsafe {
            mgr.open_channel.unwrap()(mgr, peer_id, channel_id, reliable.into())
        })
        .to_result()
    }

    /// Close the connection to a given user by peer ID on the given channel.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#closechannel)
    pub fn close_channel(
        &self,
        peer_id: NetworkPeerID,
        channel_id: NetworkChannelID,
    ) -> Result<()> {
        self.with_network_manager(|mgr| unsafe {
            mgr.close_channel.unwrap()(mgr, peer_id, channel_id)
        })
        .to_result()
    }

    /// Sends data to a given peer ID through the given channel.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/networking#sendmessage)
    pub fn send_message(
        &self,
        peer_id: NetworkPeerID,
        channel_id: NetworkChannelID,
        buffer: impl AsRef<[u8]>,
    ) -> Result<()> {
        let buffer = buffer.as_ref();

        debug_assert!(u32::try_from(buffer.len()).is_ok());

        self.with_network_manager(|mgr| unsafe {
            mgr.send_message.unwrap()(
                mgr,
                peer_id,
                channel_id,
                // XXX: *mut should be *const
                buffer.as_ptr() as *mut u8,
                // XXX: u32 should be u64
                buffer.len().try_into().unwrap_or(u32::max_value()),
            )
        })
        .to_result()
    }
}
