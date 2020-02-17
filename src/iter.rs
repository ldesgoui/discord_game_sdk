//! Iterator facilities

use crate::Discord;

/// An Iterator to acquire collections.
pub struct Collection<'d, T> {
    discord: &'d Discord,
    getter: Box<dyn FnMut(&Discord, u32) -> T>,
    count: u32,
    index: u32,
    back_index: u32,
}

impl<'d, T> Collection<'d, T> {
    pub(crate) fn new(
        discord: &'d Discord,
        getter: Box<dyn FnMut(&Discord, u32) -> T>,
        count: u32,
    ) -> Self {
        Self {
            discord,
            getter,
            count,
            index: 0,
            back_index: 0,
        }
    }
}

impl<T> Iterator for Collection<'_, T> {
    type Item = T;

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

impl<T> DoubleEndedIterator for Collection<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index + self.back_index < self.count {
            self.back_index += 1;
            Some((self.getter)(self.discord, self.count - self.back_index))
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for Collection<'_, T> {}

impl<T> std::iter::FusedIterator for Collection<'_, T> {}

impl<T> std::fmt::Debug for Collection<'_, T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Collection")
            .field("discord", &self.discord)
            .field("getter", &(..))
            .field("count", &self.count)
            .field("index", &self.index)
            .field("back_index", &self.back_index)
            .finish()
    }
}
