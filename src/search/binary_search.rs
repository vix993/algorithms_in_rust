// Should be O ( log n ) -> Logarithmic Time / Sublinear

pub fn binary_search<T: PartialOrd>(array: &[T], value: T) -> Option<usize> {
	let mut low = 0;
	let mut high = array.len() - 1;

	return loop {
		let mid = (low + high) / 2;
		if array[mid] < value {
			low = mid + 1;
		} else if array[mid] > value {
			high = mid - 1;
		} else {
			break Some(mid);
		}
		if low > high {
			break None;
		}
	}
}
