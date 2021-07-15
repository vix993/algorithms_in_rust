use algorithms_in_rust::search::{linear_search, binary_search};

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        assert_eq!(linear_search(&[1, 2, 3, 4, 5], 2), Some(1));
        assert_eq!(linear_search(&[1, 2, 3, 4, 5], 6), None);
        assert_eq!(linear_search(&[1, 2, 3, 4, 5], 5), Some(4));

    }
    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(&[1, 2, 3, 4, 5], 2), Some(1));
        assert_eq!(binary_search(&[1, 2, 3, 4, 5], 6), None);
        assert_eq!(binary_search(&[1, 2, 3, 4, 5], 5), Some(4));
    }
}