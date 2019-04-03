use crate::prelude::*;

/// # Lobbies
impl<'a> Discord<'a> {
    pub fn lobby_events_reader(&mut self) -> shrev::ReaderId<event::Lobby> {
        self.lobby_channel.register_reader()
    }

    pub fn lobby_events(
        &self,
        reader: &mut shrev::ReaderId<event::Lobby>,
    ) -> shrev::EventIterator<event::Lobby> {
        self.lobby_channel.read(reader)
    }
}
