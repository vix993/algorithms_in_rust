// O ( n ) -> Linear Time

pub fn linear_search(array: &[i32], value: i32) -> Option<usize> {
    for i in 0..array.len() {
        if array[i] == value {
           return Some(i);
        }
    }
    None
}
