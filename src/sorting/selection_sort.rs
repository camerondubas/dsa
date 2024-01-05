pub fn selection_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    if arr.is_empty() {
        return arr;
    }

    for starting_idx in 0..arr.len() - 1 {
        let mut lowest_index = starting_idx;

        // Skip the first element since we already stored it for comparison
        for idx in (starting_idx + 1)..arr.len() {
            if arr[idx] < arr[lowest_index] {
                lowest_index = idx;
            }
        }

        arr.swap(starting_idx, lowest_index);
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsorted() {
        let mut arr = [5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        selection_sort(&mut arr);
        assert_eq!(arr, []);
    }
}
