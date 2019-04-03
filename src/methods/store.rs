use crate::prelude::*;

/// # Store
impl<'a> Discord<'a> {
    pub fn store_events_reader(&mut self) -> shrev::ReaderId<event::Store> {
        self.store_channel.register_reader()
    }

    pub fn store_events(
        &self,
        reader: &mut shrev::ReaderId<event::Store>,
    ) -> shrev::EventIterator<event::Store> {
        self.store_channel.read(reader)
    }
}
