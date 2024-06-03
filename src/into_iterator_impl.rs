use crate::OptArrayVec;

pub struct IntoIter<const CAP: usize, T> {
	parent: OptArrayVec<CAP, T>,
	index: usize,
	len: usize,
}

impl<const CAP: usize, T> OptArrayVec<CAP, T> {
	pub fn into_iter(self) -> IntoIter<CAP, T> {
		IntoIter {
			len: self.len(),
			parent: self,
			index: 0,
		}
	}
}
impl<const CAP: usize, T> Iterator for IntoIter<CAP, T> {
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

