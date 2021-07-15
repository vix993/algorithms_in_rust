use algorithms_in_rust::sort::bubble_sort;

#[cfg(test)]
mod sort_tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut test_small_sample_middle: Vec<i32> = vec![1, 3, 2, 4, 5];
	let sample_middle_size: usize = test_small_sample_middle.len();
	let small_sample_middle_expected: Vec<i32> = vec![1, 2, 3, 4, 5];

        assert_eq!(bubble_sort(&mut test_small_sample_middle, sample_middle_size), &small_sample_middle_expected);

	let mut test_small_sample_front: Vec<i32> = vec![100, 1, 2, 3, 4];
	let sample_front_size: usize = test_small_sample_front.len();
	let small_sample_front_expected: Vec<i32> = vec![1, 2, 3, 4, 100];

        assert_eq!(bubble_sort(&mut test_small_sample_front, sample_front_size), &small_sample_front_expected);

	let mut test_mid_random_sample: Vec<i32> = vec![100, 22, 5, 65, 87, 45, 39, 101, 27, 27, 65, 89];
	let sample_mid_random_size: usize = test_mid_random_sample.len();
	let mut test_mid_random_sample_expected: Vec<i32> = vec![100, 22, 5, 65, 87, 45, 39, 101, 27, 27, 65, 89];
	test_mid_random_sample_expected.sort();

        assert_eq!(bubble_sort(&mut test_mid_random_sample, sample_mid_random_size), &test_mid_random_sample_expected);
    }
}
