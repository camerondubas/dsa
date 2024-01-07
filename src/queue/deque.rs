use std::collections::LinkedList;

#[derive(Default)]
pub struct Deque<T> {
    queue: LinkedList<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            queue: LinkedList::new(),
        }
    }

    pub fn enqueue_front(&mut self, data: T) {
        self.queue.push_front(data);
    }
    pub fn enqueue_back(&mut self, data: T) {
        self.queue.push_back(data);
    }

    pub fn dequeue_front(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn dequeue_back(&mut self) -> Option<T> {
        self.queue.pop_back()
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.queue.front()
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.queue.back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_front() {
        let mut queue = Deque::new();
        queue.enqueue_front('a');
        queue.enqueue_front('b');
        queue.enqueue_front('c');
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.peek_front(), Some(&'c'));
    }

    #[test]
    fn enqueue_back() {
        let mut queue = Deque::new();
        queue.enqueue_back('a');
        queue.enqueue_back('b');
        queue.enqueue_back('c');
        assert_eq!(queue.size(), 3);
        assert_eq!(queue.peek_front(), Some(&'a'));
    }

    #[test]
    fn dequeue_front() {
        let mut queue = Deque::new();
        queue.enqueue_back('a');
        queue.enqueue_back('b');
        queue.enqueue_back('c');

        let val = queue.dequeue_front();

        assert_eq!(queue.size(), 2);
        assert_eq!(val, Some('a'));
        assert_eq!(queue.peek_front(), Some(&'b'));
        assert_eq!(queue.peek_back(), Some(&'c'));
    }

    #[test]
    fn dequeue_back() {
        let mut queue = Deque::new();
        queue.enqueue_back('a');
        queue.enqueue_back('b');
        queue.enqueue_back('c');

        let val = queue.dequeue_back();

        assert_eq!(queue.size(), 2);
        assert_eq!(val, Some('c'));
        assert_eq!(queue.peek_front(), Some(&'a'));
        assert_eq!(queue.peek_back(), Some(&'b'));
    }

    #[test]
    fn complex() {
        let mut queue = Deque::new();
        queue.enqueue_back('a');
        queue.enqueue_back('b');
        queue.enqueue_back('c');

        let val = queue.dequeue_front();

        assert_eq!(queue.size(), 2);
        assert_eq!(val, Some('a'));

        queue.enqueue_back('d');
        queue.enqueue_front('e');

        let val = queue.dequeue_front();

        assert_eq!(queue.size(), 3);
        assert_eq!(val, Some('e'));

        let val = queue.dequeue_back();

        assert_eq!(queue.size(), 2);
        assert_eq!(val, Some('d'));
        assert_eq!(queue.peek_front(), Some(&'b'));
        assert_eq!(queue.peek_back(), Some(&'c'));
    }

    #[test]
    fn empty() {
        let queue: Deque<char> = Deque::new();
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.peek_front(), None);
        assert_eq!(queue.peek_back(), None);
    }

    #[test]
    fn dequeue_empty() {
        let mut queue: Deque<char> = Deque::new();

        let val = queue.dequeue_back();

        assert_eq!(val, None);
    }
}
