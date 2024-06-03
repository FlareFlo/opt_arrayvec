use std::ops::Index;
use crate::OptArrayVec;

impl<const CAP: usize, T> Default for OptArrayVec<CAP, T> {
	fn default() -> Self {
		Self::new()
	}
}

impl <const CAP: usize, T>FromIterator<T> for OptArrayVec<CAP, T> {
	/// Create a vec from an iterator.
	///
	/// # Panics
	/// When the iterator yields more elements than CAP
	fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
		let mut new = Self::new();
		new.extend(iter);
		new
	}
}

impl<const CAP: usize, T> Extend<T> for OptArrayVec<CAP, T> {
	/// Appends iterator to Self
	///
	/// # Panics
	/// When the iterator yields more elements than there is remaining space
	fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
		iter.into_iter().for_each(|e|self.push(e));
	}
}

impl<const CAP: usize, T> Index<usize> for OptArrayVec<CAP, T> {
	type Output = Option<T>;

	fn index(&self, index: usize) -> &Self::Output {
		for (i, elem) in self.inner.iter().enumerate() {
			if i == index {
				return elem
			}
		}
		&None
	}
}