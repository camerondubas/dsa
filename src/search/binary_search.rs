pub fn binary_search<T: PartialEq + PartialOrd>(haystack: &[T], needle: T) -> Option<usize> {
    if haystack.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = haystack.len() - 1;

    while low < high {
        let mid = (high - low) / 2 + low;
        if haystack[mid] == needle {
            return Some(mid);
        }

        if needle > haystack[mid] {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn present_even() {
        let haystack = ['a', 'b', 'c', 'd'];
        let needle = 'c';

        let idx = binary_search(&haystack, needle);

        assert_eq!(idx, Some(2));
    }

    #[test]
    fn not_present_even() {
        let haystack = ['a', 'b', 'c', 'd'];
        let needle = 'e';

        let idx = binary_search(&haystack, needle);

        assert_eq!(idx, None);
    }

    #[test]
    fn present_odd() {
        let haystack = ['a', 'b', 'c', 'd', 'e'];
        let needle = 'c';

        let idx = binary_search(&haystack, needle);

        assert_eq!(idx, Some(2));
    }

    #[test]
    fn not_present_odd() {
        let haystack = ['a', 'b', 'c', 'd', 'e'];
        let needle = 'f';

        let idx = binary_search(&haystack, needle);

        assert_eq!(idx, None);
    }

    #[test]
    fn empty() {
        let haystack = [];
        let needle = 'e';

        let idx = binary_search(&haystack, needle);

        assert_eq!(idx, None);
    }
}
