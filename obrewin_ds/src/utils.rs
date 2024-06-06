/// Iterator struct wrapper for general purposes.
/// This struct is used when you want specific trait to be object safe,
/// but want to return an iterator in that trait's method.
pub struct WrappedIterator<'it, Item> {
    prev_iterator: Box<dyn Iterator<Item = Item> + 'it>,
}

impl<'it, Item> WrappedIterator<'it, Item> {
    /// Create new instance of wrapped iterator.
    pub fn new<T>(prev_iterator: T) -> Self
    where
        T: Iterator<Item = Item> + 'it,
    {
        Self {
            prev_iterator: Box::new(prev_iterator),
        }
    }
}

impl<'it, Item> Iterator for WrappedIterator<'it, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev_iterator.next()
    }
}

/// Convenience trait to wrap iterators.
pub trait WrapIterator<'s, Item> {
    /// Wrap current struct into wrapped iterator.
    fn wrap_iter(self) -> WrappedIterator<'s, Item>;
}

impl<'s, Item, It> WrapIterator<'s, Item> for It
where
    It: IntoIterator<Item = Item> + 's,
{
    fn wrap_iter(self) -> WrappedIterator<'s, Item> {
        WrappedIterator::new(self.into_iter())
    }
}
