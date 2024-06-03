use crate::OptArrayVec;

impl<const CAP: usize, T> Default for OptArrayVec<CAP, T> {
	fn default() -> Self {
		Self::new()
	}
}
