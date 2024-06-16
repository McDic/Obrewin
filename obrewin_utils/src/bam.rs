use std::future::Future;

/// BAM is an abbreviated name of "BoxedAsyncMethod".
/// It represents an async method that gets one
/// parameter and returns a boxed async future.
pub type BAM<'se, T, R> = Box<dyn Fn(T) -> Box<dyn Future<Output = R> + 'se> + 'se>;

/// Similar to `Into<BAM<..>>`.
pub trait IntoBAM<'se, T, R> {
    /// Convert this object into `BAM`.
    fn into_bam(self) -> BAM<'se, T, R>;
}

impl<'se, T, R, C, Fut> IntoBAM<'se, T, R> for C
where
    C: Fn(T) -> Fut + 'se,
    Fut: Future<Output = R> + 'se,
{
    fn into_bam(self) -> BAM<'se, T, R> {
        Box::new(move |parameter: T| Box::new(self(parameter)))
    }
}
