use std::ops::Index;
use crate::OptArrayVec;

pub struct Iter<'a, const CAP: usize, T> {
	parent: &'a OptArrayVec<CAP, T>,
	index: usize,
	len: usize,
}

impl<const CAP: usize, T> OptArrayVec<CAP, T> {
	pub fn iter(&self) -> Iter<CAP, T> {
		Iter {
			parent: self,
			index: 0,
			len: self.len(),
		}
	}
}
impl<'a, const CAP: usize, T> Iterator for Iter<'a, CAP, T> {
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

