use crate::OptArrayVec;

pub struct Iter<'a, T> {
	inner: core::slice::Iter<'a, Option<T>>,
}

impl<T, const CAP: usize> OptArrayVec<T, CAP> {
	pub fn iter(&self) -> Iter<'_, T> {
		Iter {
			inner: self.inner.iter(),
		}
	}
}

impl<'a, T, const CAP: usize> IntoIterator for &'a OptArrayVec<T, CAP> {
	type IntoIter = Iter<'a, T>;
	type Item = &'a T;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		match self.inner.next() {
			Some(Some(item)) => Some(item),
			Some(None) | None => None,
		}
	}
}
