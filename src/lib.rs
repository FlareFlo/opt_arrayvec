mod impls;
#[cfg(test)]
mod tests;
mod trait_impl;
pub mod iterator_impl;
pub mod into_iterator_impl;

/// A vector-like container that does not store its length directly, but rather retrieves them through inner options
/// I highly discourage using this if `size_of::<Yourtype>` != `size_of::<Option<Yourtype>>`
//
// # Guidance for the Layout of this type:
// The inner array must never be fragmented/sparse.
// It must either be filled with None, or start with a contiguous section of Some, followed exclusively by None.
// Not covering this requirement means member functions will unexpectedly panic or produce UB in the future.
#[derive(Debug, Copy, Clone)]
pub struct OptArrayVec<const N: usize, T> {
	inner: [Option<T>; N],
}
