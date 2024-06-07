/// Creates an `OptArrayVec` from the arguments.
#[macro_export]
macro_rules! arrayvec {
	() => {
		$crate::OptArrayVec::new()
	};
	($elem:expr; $n:expr) => {{
		let mut vec = $crate::OptArrayVec::new();
		for _ in 0..$n {
			vec.push($elem.clone())
		}
		vec
	}};
	($elem:expr; $n:expr, $cap:expr) => {{
		let mut vec: OptArrayVec<_, $cap> = $crate::OptArrayVec::new();
		for _ in 0..$n {
			vec.push($elem.clone())
		}
		vec
	}};
	($($value:expr),*) => {{
		let mut vec = $crate::OptArrayVec::new();
		$(
			vec.push($value);
		)*
		vec
	}};
	($($value:expr),*; $cap:expr) => {{
		let mut vec: OptArrayVec<_, $cap> = $crate::OptArrayVec::new();
		$(
			vec.push($value);
		)*
		vec
	}};
}

#[cfg(test)]
mod test {
	use crate::OptArrayVec;

	#[test]
	fn empty() {
		let _vec: OptArrayVec<u8, 7> = arrayvec![];
	}

	#[test]
	fn splat_without_cap() {
		let vec: OptArrayVec<i32, 15> = arrayvec![42; 13];
		assert_eq!(vec.len(), 13);
	}

	#[test]
	fn splat_with_cap() {
		let vec = arrayvec![42; 13, 15];
		assert_eq!(vec.len(), 13);
	}

	#[test]
	fn distinct_without_cap() {
		let vec: OptArrayVec<i32, 6> = arrayvec![1, 2, 3, 4, 5];
		assert_eq!(vec.len(), 5);
	}

	#[test]
	fn distinct_with_cap() {
		let vec = arrayvec![1, 2, 3, 4, 5; 6];
		assert_eq!(vec.len(), 5);
	}
}
