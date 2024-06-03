use std::ops::Index;

use crate::OptArrayVec;

pub struct Iter<'a, T, const CAP: usize> {
	parent: &'a OptArrayVec<T, CAP>,
	index: usize,
	len: usize,
}

impl<T, const CAP: usize> OptArrayVec<T, CAP> {
	pub fn iter(&self) -> Iter<'_, T, CAP> {
		Iter {
			parent: self,
			index: 0,
			len: self.len(),
		}
	}
}

impl<'a, T, const CAP: usize> IntoIterator for &'a OptArrayVec<T, CAP> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T, CAP>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, T, const CAP: usize> Iterator for Iter<'a, T, CAP> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index < self.len {
			let ret = self.parent.index(self.index);
			self.index += 1;
			Some(ret)
		} else {
			None
		}
	}
}
