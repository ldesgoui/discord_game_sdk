use crate::prelude::*;

/// # Networking
impl<'a> Discord<'a> {
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
