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

	/// Attempts to swap two elements
	/// Returns error when one of the indices is out of bounds (either `Self::len()` or `Self::N`)
	pub fn try_swap(&mut self, a: usize, b: usize) -> Option<()> {
		// Perform bounds check
		let _ = self.get(a)?;
		let _ = self.get(b)?;

		self.inner.swap(a, b);
		Some(())
	}

	/// Swaps two elements
	///
	/// # Panics
	/// When either index is out of bounds see. Use `Self::try_swap` for fallible swapping
	pub fn swap(&mut self, a: usize, b: usize) {
		self.try_swap(a, b).unwrap();
	}

	/// Attempts to get element at index
	pub fn get(&self, index: usize) -> Option<&T> {
		self.inner.get(index)?.as_ref()
	}

	/// Attempts to get mutable element at index
	pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.inner.get_mut(index)?.as_mut()
	}

	/// Takes value at index, replacing it with last element if any remains
	pub fn try_swap_remove(&mut self, index: usize) -> Option<T> {
		let mut last = self.pop()?;

		if self.is_empty() {
			return Some(last);
		}

		let to_swap = self.get_mut(index)?;
		core::mem::swap(&mut last, to_swap);
		Some(last)
	}

	/// Takes value at index, replacing it with last element if any remains
	///
	/// # Panics
	/// When the index is out of bounds, or there is only one element remaining
	pub fn swap_remove(&mut self, index: usize) -> T {
		self.try_swap_remove(index).unwrap()
	}
}
