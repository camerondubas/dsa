pub fn insertion_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    for mut selected_idx in 1..arr.len() {
        for idx in (0..selected_idx).rev() {
            if arr[idx] > arr[selected_idx] {
                arr.swap(idx, selected_idx);
                selected_idx = idx;
                continue;
            }

            break;
        }
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsorted() {
        let mut arr = [5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        assert_eq!(arr, []);
    }
}
