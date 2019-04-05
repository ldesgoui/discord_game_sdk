use crate::prelude::*;

/// # Networking
impl<'a> Discord<'a> {
    pub fn peer_id(&mut self) -> u64 {
        let mut peer_id = 0;

        unsafe { ffi!(self.get_network_manager().get_peer_id(&mut peer_id)) }

        peer_id
    }

    pub fn flush_network(&mut self) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().flush()) }.to_result()
    }

    pub fn open_peer<S>(&mut self, peer_id: u64, route: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let route = CString::new(route.as_ref()).unwrap();

        unsafe {
            ffi!(self
                .get_network_manager()
                .open_peer(peer_id, route.as_ptr()))
        }
        .to_result()
    }

    pub fn update_peer<S>(&mut self, peer_id: u64, route: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let route = CString::new(route.as_ref()).unwrap();

        unsafe {
            ffi!(self
                .get_network_manager()
                .update_peer(peer_id, route.as_ptr()))
        }
        .to_result()
    }

    pub fn close_peer(&mut self, peer_id: u64) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().close_peer(peer_id)) }.to_result()
    }

    pub fn open_channel(&mut self, peer_id: u64, chan_id: u8, reliable: bool) -> Result<()> {
        unsafe {
            ffi!(self
                .get_network_manager()
                .open_channel(peer_id, chan_id, reliable))
        }
        .to_result()
    }

    pub fn close_channel(&mut self, peer_id: u64, chan_id: u8) -> Result<()> {
        unsafe { ffi!(self.get_network_manager().close_channel(peer_id, chan_id)) }.to_result()
    }

    pub fn send_message(&mut self, peer_id: u64, chan_id: u8, buf: &[u8]) -> Result<()> {
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

    pub fn network_events_reader(&mut self) -> shrev::ReaderId<event::Network> {
        self.network_channel.register_reader()
    }

    pub fn network_events(
        &self,
        reader: &mut shrev::ReaderId<event::Network>,
    ) -> shrev::EventIterator<event::Network> {
        self.network_channel.read(reader)
    }
}
