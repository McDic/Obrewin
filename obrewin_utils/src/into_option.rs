/// Convenient trait to convert to `Option<T>`.
/// `IntoOption<T> for F` is automatically implemented
/// if `TryInto<T> for F` is implemented.
pub trait IntoOption<T> {
    fn into_option(self) -> Option<T>;
}

impl<T, F> IntoOption<T> for F
where
    T: TryFrom<F>,
{
    fn into_option(self) -> Option<T> {
        self.try_into().ok()
    }
}
