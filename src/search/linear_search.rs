pub fn linear_search<T: PartialEq>(haystack: &[T], needle: T) -> Option<usize> {
    #[allow(clippy::manual_find)]
    for (idx, _) in haystack.iter().enumerate() {
        if haystack[idx] == needle {
            return Some(idx);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::linear_search;

    #[test]
    fn present_even() {
        let haystack = ['a', 'b', 'c', 'd'];
        let needle = 'c';

        let idx = linear_search(&haystack, needle);

        assert_eq!(idx, Some(2));
    }

    #[test]
    fn not_present_even() {
        let haystack = ['a', 'b', 'c', 'd'];
        let needle = 'e';

        let idx = linear_search(&haystack, needle);

        assert_eq!(idx, None);
    }

    #[test]
    fn present_odd() {
        let haystack = ['a', 'b', 'c', 'd', 'e'];
        let needle = 'c';

        let idx = linear_search(&haystack, needle);

        assert_eq!(idx, Some(2));
    }

    #[test]
    fn not_present_odd() {
        let haystack = ['a', 'b', 'c', 'd', 'e'];
        let needle = 'f';

        let idx = linear_search(&haystack, needle);

        assert_eq!(idx, None);
    }

    #[test]
    fn empty() {
        let haystack = [];
        let needle = 'e';

        let idx = linear_search(&haystack, needle);

        assert_eq!(idx, None);
    }
}
