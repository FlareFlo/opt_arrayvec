use crate::OptArrayVec;

impl<T, const CAP: usize> OptArrayVec<T, CAP> {
	// Binds T, therefore must be in the global context, meaning it cannot be moved into Self::new
	const ARRAY_REPEAT_VALUE: Option<T> = None;

	/// Returns capacity
	pub fn cap(&self) -> usize {
		CAP
	}

	/// Amount of elements in the vec
	///
	/// # Complexity
	/// Runtime is O(CAP) worst case
	pub fn len(&self) -> usize {
		self.inner.iter().filter(|x| x.is_some()).count()
	}

	pub fn is_empty(&self) -> bool {
		if CAP == 0 {
			true
		} else {
			self.inner[0].is_none()
		}
	}

	/// Create a new empty vec
	/// CAP must be provided for how many elements can be stored
	#[must_use]
	pub const fn new() -> Self {
		Self {
			inner: [Self::ARRAY_REPEAT_VALUE; CAP],
		}
	}

	/// Pushes new element at the back of the vec
	///
	///  # Complexity
	///  Runtime is O(CAP) worst case
	///
	///  # Panics
	/// When the len == CAP
	pub fn push(&mut self, element: T) {
		for i in 0..CAP {
			let at = &mut self.inner[i];
			if at.is_none() {
				*at = Some(element);
				return;
			}
		}
		panic!("OptArrayVec is full!")
	}

	/// Removes all elements from the vec
	pub fn clear(&mut self) {
		*self = Self::new();
	}

	/// Removes last element from the vec and return it
	/// If the vec is empty None is returned instead
	pub fn pop(&mut self) -> Option<T> {
		for i in (0..CAP).rev() {
			let at = &mut self.inner[i];
			if let Some(val) = at.take() {
				return Some(val);
			}
		}
		None
	}

	/// Returns contents as they are stored internally
	pub fn into_inner(self) -> [Option<T>; CAP] {
		self.inner
	}

	/// Returns capacity still remaining
	pub fn remaining_capacity(&self) -> usize {
		CAP - self.len()
	}
}
