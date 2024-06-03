use std::ops::{Index, IndexMut};
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
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		for (i, elem) in self.inner.iter().enumerate() {
			if i == index {
				return elem.as_ref().expect("Infallible")
			}
		}
		panic!("Index {index} out of bounds for length {}", self.len())
	}
}

impl<const CAP: usize, T> IndexMut<usize> for OptArrayVec<CAP, T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		let mut len = 0;
		for (i, elem) in self.inner.iter_mut().enumerate() {
			if i == index {
				return elem.as_mut().expect("Infallible")
			}
			len = index;
		}
		panic!("Index {index} out of bounds for length {len}")
	}
}