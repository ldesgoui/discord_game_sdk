use crate::Discord;

/// A reusable Iterator for the SDK's methods to acquire collections.
pub struct GenericIter<'a: 'b, 'b, T> {
    discord: &'b Discord<'a>,
    getter: Box<dyn FnMut(&Discord, i32) -> T>,
    count: i32,
    index: i32,
    back_index: i32,
}

impl<'a: 'b, 'b, T> GenericIter<'a, 'b, T> {
    pub(crate) fn new(
        discord: &'b Discord<'a>,
        getter: Box<dyn FnMut(&Discord, i32) -> T>,
        count: i32,
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

impl<T> Iterator for GenericIter<'_, '_, T> {
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

impl<T> DoubleEndedIterator for GenericIter<'_, '_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index + self.back_index < self.count {
            self.back_index += 1;
            Some((self.getter)(self.discord, self.count - self.back_index))
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for GenericIter<'_, '_, T> {}

impl<T> std::iter::FusedIterator for GenericIter<'_, '_, T> {}
