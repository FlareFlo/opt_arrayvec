mod impls;
#[cfg(test)]
mod tests;
mod trait_impl;
pub mod iterator_impl;

/// A vector-like container that does not store its length directly, but rather retrieves them through inner options
#[derive(Debug, Copy, Clone)]
pub struct OptArrayVec<const N: usize, T> {
	inner: [Option<T>; N],
}
