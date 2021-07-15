// Quadratic Time

pub fn bubble_sort_recursive(to_sort: &mut Vec<i32>, vec_size: usize) -> &Vec<i32> {
    if vec_size == 1 {
        return to_sort;
    }
//     println!("{:?}", to_sort);

    for i in 0..vec_size - 1 {
        if to_sort[i] > to_sort[i + 1] {
            let smaller = to_sort.remove(i + 1);
            to_sort.insert(i, smaller);
        }
    }

    bubble_sort_recursive(to_sort, vec_size - 1)
}
