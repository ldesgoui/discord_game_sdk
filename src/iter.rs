use crate::Discord;

pub(crate) struct GenericIter<'a, T, F: FnMut(&mut Discord, i32) -> T> {
    discord: &'a mut Discord<'a>,
    getter: F,
    count: i32,
    index: i32,
    back_index: i32,
}

impl<'a, T, F> GenericIter<'a, T, F>
where
    F: FnMut(&mut Discord, i32) -> T,
{
    pub(crate) fn new(discord: &'a mut Discord<'a>, getter: F, count: i32) -> Self {
        Self {
            discord,
            getter,
            count,
            index: 0,
            back_index: 0,
        }
    }
}

impl<T, F> Iterator for GenericIter<'_, T, F>
where
    F: FnMut(&mut Discord, i32) -> T,
{
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

impl<T, F> DoubleEndedIterator for GenericIter<'_, T, F>
where
    F: FnMut(&mut Discord, i32) -> T,
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

impl<T, F> ExactSizeIterator for GenericIter<'_, T, F> where F: FnMut(&mut Discord, i32) -> T {}

impl<T, F> std::iter::FusedIterator for GenericIter<'_, T, F> where F: FnMut(&mut Discord, i32) -> T {}
