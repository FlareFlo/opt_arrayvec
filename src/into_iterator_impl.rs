use crate::OptArrayVec;

pub struct IntoIter<T, const CAP: usize> {
	parent: OptArrayVec<T, CAP>,
	index: usize,
	len: usize,
}

impl<T, const CAP: usize> IntoIterator for OptArrayVec<T, CAP> {
	type Item = T;
	type IntoIter = IntoIter<T, CAP>;

	fn into_iter(self) -> IntoIter<T, CAP> {
		IntoIter {
			len: self.len(),
			parent: self,
			index: 0,
		}
	}
}

impl<T, const CAP: usize> Iterator for IntoIter<T, CAP> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if self.index < self.len {
			let ret = self.parent.inner[self.index].take();
			self.index += 1;
			ret
		} else {
			None
		}
	}
}
