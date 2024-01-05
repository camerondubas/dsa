pub fn bubble_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    if arr.is_empty() {
        return arr;
    }

    let mut unsorted_index = arr.len() - 1;

    while unsorted_index > 0 {
        for idx in 0..unsorted_index {
            if arr[idx] > arr[idx + 1] {
                arr.swap(idx, idx + 1);
            }
        }

        unsorted_index -= 1;
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsorted() {
        let mut arr = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        bubble_sort(&mut arr);
        assert_eq!(arr, []);
    }
}
