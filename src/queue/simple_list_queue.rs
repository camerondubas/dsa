use std::collections::LinkedList;

#[derive(Default)]
pub struct SimpleListQueue<T> {
    queue: LinkedList<T>,
}

impl<T> SimpleListQueue<T> {
    pub fn new() -> Self {
        SimpleListQueue {
            queue: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, data: T) {
        self.queue.push_back(data);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue() {
        let mut queue = SimpleListQueue::new();
        queue.enqueue('a');
        queue.enqueue('b');
        queue.enqueue('c');
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn peek() {
        let mut queue = SimpleListQueue::new();
        queue.enqueue('a');
        queue.enqueue('b');
        queue.enqueue('c');
        assert_eq!(queue.peek(), Some(&'a'));
    }

    #[test]
    fn dequeue() {
        let mut queue = SimpleListQueue::new();
        queue.enqueue('a');
        queue.enqueue('b');
        queue.enqueue('c');

        let val = queue.dequeue();

        assert_eq!(queue.size(), 2);
        assert_eq!(val, Some('a'));
        assert_eq!(queue.peek(), Some(&'b'));
    }

    #[test]
    fn empty() {
        let queue: SimpleListQueue<char> = SimpleListQueue::new();
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn dequeue_empty() {
        let mut queue: SimpleListQueue<char> = SimpleListQueue::new();

        let val = queue.dequeue();

        assert_eq!(val, None);
    }
}
