use algorithms_in_rust::sort::bubble_sort;

fn main() {
    let mut test_small_sample: Vec<i32> = vec![1, 3, 2, 4, 5];
    let sample_size: usize = test_small_sample.len();
	let small_sample_expected: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Hello, world!");

    assert_eq!(bubble_sort(&mut test_small_sample, sample_size), &small_sample_expected);
}
