use crate::OptArrayVec;

pub struct IntoIter<T, const CAP: usize> {
	inner: std::array::IntoIter<Option<T>, CAP>,
}

impl<T, const CAP: usize> IntoIterator for OptArrayVec<T, CAP> {
	type IntoIter = IntoIter<T, CAP>;
	type Item = T;

	fn into_iter(self) -> IntoIter<T, CAP> {
		IntoIter {
			inner: self.inner.into_iter(),
		}
	}
}

impl<T, const CAP: usize> Iterator for IntoIter<T, CAP> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self.inner.next() {
			Some(Some(item)) => Some(item),
			Some(None) | None => None,
		}
	}
}
