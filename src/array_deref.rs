use core::{
	num::{
		NonZeroI128,
		NonZeroI16,
		NonZeroI32,
		NonZeroI64,
		NonZeroI8,
		NonZeroIsize,
		NonZeroU128,
		NonZeroU16,
		NonZeroU32,
		NonZeroU64,
		NonZeroU8,
		NonZeroUsize,
	},
	ops::{Deref, DerefMut},
	ptr::NonNull,
	slice,
};

use crate::{impl_nonzero, OptArrayVec};

impl<T, const N: usize> Deref for OptArrayVec<T, N>
where
	T: SafeDeref,
{
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		let len = self.len();

		// Safety:
		// This is only valid because T is safe to downcast from Option<T> when it is Some.
		unsafe {
			let ptr = self.inner.as_ptr().cast();
			slice::from_raw_parts(ptr, len)
		}
	}
}

impl<T, const N: usize> DerefMut for OptArrayVec<T, N>
where
	T: SafeDeref,
{
	fn deref_mut(&mut self) -> &mut [T] {
		let len = self.len();

		// Safety:
		// This is only valid because T is safe to downcast from Option<T> when it is Some.
		unsafe {
			let ptr = self.inner.as_ptr().cast_mut().cast();
			slice::from_raw_parts_mut(ptr, len)
		}
	}
}

trait SafeDeref {}

#[cfg(feature = "std")]
impl<U> SafeDeref for std::boxed::Box<U> where U: Sized {}

impl<U> SafeDeref for &U where U: Sized {}

impl<U> SafeDeref for &mut U where U: Sized {}

impl<U> SafeDeref for NonNull<U> where U: Sized {}

impl_nonzero!(
	NonZeroI8
	NonZeroI16
	NonZeroI32
	NonZeroI64
	NonZeroI128
	NonZeroIsize
	NonZeroU8
	NonZeroU16
	NonZeroU32
	NonZeroU64
	NonZeroU128
	NonZeroUsize
);

#[macro_export]
macro_rules! impl_nonzero {
    ($( $t:ty )*) => {
		$(
			impl SafeDeref for $t {}
		)*
	};
}

#[cfg(test)]
mod test {
	use core::{num::NonZeroU32, ops::Deref};

	use crate::OptArrayVec;

	#[test]
	fn one() {
		let vec: OptArrayVec<_, 5> = OptArrayVec::from_iter([NonZeroU32::new(3).unwrap()]);
		assert_eq!(vec.deref().len(), 1);
		assert_eq!(vec.deref()[0].get(), 3);
	}

	#[test]
	fn full() {
		let vec: OptArrayVec<_, 3> = OptArrayVec::from_iter([
			NonZeroU32::new(3).unwrap(),
			NonZeroU32::new(42).unwrap(),
			NonZeroU32::new(100).unwrap(),
		]);
		assert_eq!(vec.deref().len(), 3);
		assert_eq!(
			vec.deref(),
			&[
				NonZeroU32::new(3).unwrap(),
				NonZeroU32::new(42).unwrap(),
				NonZeroU32::new(100).unwrap()
			]
		);
	}
}
