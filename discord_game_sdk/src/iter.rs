pub(crate) struct Collection<'r, I> {
    getter: Box<dyn 'r + Fn(u32) -> I>,
    count: u32,
    index: u32,
    back_index: u32,
}

impl<'r, I> Collection<'r, I> {
    pub(crate) fn new(getter: Box<dyn 'r + Fn(u32) -> I>, count: u32) -> Self {
        Self {
            getter,
            count,
            index: 0,
            back_index: 0,
        }
    }
}

impl<I> Iterator for Collection<'_, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.back_index < self.count {
            self.index += 1;
            Some((self.getter)(self.index - 1))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
}

impl<I> DoubleEndedIterator for Collection<'_, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index + self.back_index < self.count {
            self.back_index += 1;
            Some((self.getter)(self.count - self.back_index))
        } else {
            None
        }
    }
}

impl<I> ExactSizeIterator for Collection<'_, I> {}

impl<I> std::iter::FusedIterator for Collection<'_, I> {}

impl<I> std::fmt::Debug for Collection<'_, I> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Collection")
            .field("getter", &(..))
            .field("count", &self.count)
            .field("index", &self.index)
            .field("back_index", &self.back_index)
            .finish()
    }
}
