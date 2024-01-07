#[derive(Default)]
pub struct SimpleVecQueue<T> {
    queue: Vec<T>,
}

impl<T> SimpleVecQueue<T> {
    pub fn new() -> Self {
        SimpleVecQueue { queue: vec![] }
    }

    pub fn enqueue(&mut self, data: T) {
        self.queue.insert(0, data);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop()
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue() {
        let mut queue = SimpleVecQueue::new();
        queue.enqueue('a');
        queue.enqueue('b');
        queue.enqueue('c');
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn peek() {
        let mut queue = SimpleVecQueue::new();
        queue.enqueue('a');
        queue.enqueue('b');
        queue.enqueue('c');
        assert_eq!(queue.peek(), Some(&'a'));
    }

    #[test]
    fn dequeue() {
        let mut queue = SimpleVecQueue::new();
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
        let queue: SimpleVecQueue<char> = SimpleVecQueue::new();
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn dequeue_empty() {
        let mut queue: SimpleVecQueue<char> = SimpleVecQueue::new();

        let val = queue.dequeue();

        assert_eq!(val, None);
    }
}
