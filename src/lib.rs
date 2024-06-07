#![warn(clippy::pedantic)]
#![warn(rust_2018_idioms)]
#![no_std]

mod impls;
mod into_iterator_impl;
mod iterator_impl;
mod macro_builder;
#[cfg(test)]
mod tests;
mod trait_impl;

pub use into_iterator_impl::IntoIter;
pub use iterator_impl::Iter;

/// A vector-like container that does not store its length directly, but rather retrieves them through inner options
/// I highly discourage using this if `size_of::<Yourtype>` != `size_of::<Option<Yourtype>>`
// # Guidance for the Layout of this type:
// The inner array must never be fragmented/sparse.
// It must either be filled with None, or start with a contiguous section of Some, followed exclusively by None.
// Not covering this requirement means member functions will unexpectedly panic or produce UB in the future.
#[derive(Debug, Copy, Clone)]
pub struct OptArrayVec<T, const N: usize> {
	inner: [Option<T>; N],
}
