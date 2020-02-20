use crate::Discord;

// An Iterator to acquire collections.
pub(crate) struct Collection<'d, E, Item, Getter: FnMut(&Discord<E>, u32) -> Item> {
    discord: &'d Discord<E>,
    getter: Getter,
    count: u32,
    index: u32,
    back_index: u32,
}

impl<'d, E, Item, Getter: FnMut(&Discord<E>, u32) -> Item> Collection<'d, E, Item, Getter> {
    pub(crate) fn new(discord: &'d Discord<E>, getter: Getter, count: u32) -> Self {
        Self {
            discord,
            getter,
            count,
            index: 0,
            back_index: 0,
        }
    }
}

impl<E, Item, Getter: FnMut(&Discord<E>, u32) -> Item> Iterator
    for Collection<'_, E, Item, Getter>
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.back_index < self.count {
            self.index += 1;
            Some((self.getter)(self.discord, self.index - 1))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
}

impl<E, Item, Getter: FnMut(&Discord<E>, u32) -> Item> DoubleEndedIterator
    for Collection<'_, E, Item, Getter>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index + self.back_index < self.count {
            self.back_index += 1;
            Some((self.getter)(self.discord, self.count - self.back_index))
        } else {
            None
        }
    }
}

impl<E, Item, Getter: FnMut(&Discord<E>, u32) -> Item> ExactSizeIterator
    for Collection<'_, E, Item, Getter>
{
}

impl<E, Item, Getter: FnMut(&Discord<E>, u32) -> Item> std::iter::FusedIterator
    for Collection<'_, E, Item, Getter>
{
}

impl<E, Item, Getter: FnMut(&Discord<E>, u32) -> Item> std::fmt::Debug
    for Collection<'_, E, Item, Getter>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Collection")
            // .field("discord", &self.discord)
            .field("getter", &(..))
            .field("count", &self.count)
            .field("index", &self.index)
            .field("back_index", &self.back_index)
            .finish()
    }
}
