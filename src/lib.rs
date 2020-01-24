/// Transform a traditional `for` loop into a parallel `for_each`
///
/// ```rust,ignore
/// for item in expr { /* body */ }
/// ```
///
/// becomes roughly:
///
/// ```rust,ignore
/// expr.into_par_iter().for_each(|item| { /* body */ });
/// ```
#[proc_macro_hack::proc_macro_hack]
pub use rayon_macro_hack::parallel;

#[doc(hidden)]
pub mod rayon_macro_impl {
    pub use rayon::iter::{IntoParallelIterator, ParallelIterator};
}
