// O ( n ) -> Linear Time

pub fn linear_search<T: PartialOrd>(array: &[T], value: T) -> Option<usize> {
    for i in 0..array.len() {
        if array[i] == value {
           return Some(i);
        }
    }
    None
}
