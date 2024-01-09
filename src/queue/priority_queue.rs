#[derive(Default)]
pub struct PriorityQueue<T> {
    queue: Vec<(T, u8)>,
}

impl<T> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, data: T, priority: u8) {
        self.queue.push((data, priority));
    }

    fn highest_priority_index(&self) -> usize {
        let mut highest_priority = 0;
        let mut highest_priority_index = 0;

        for index in 0..self.queue.len() {
            let (_, priority) = self.queue[index];
            if priority > highest_priority {
                highest_priority = priority;
                highest_priority_index = index;
            }
        }

        highest_priority_index
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            return None;
        }

        let highest_priority_index = self.highest_priority_index();
        let (data, _) = self.queue.remove(highest_priority_index);
        Some(data)
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.queue.is_empty() {
            return None;
        }

        let highest_priority_index = self.highest_priority_index();
        let (data, _) = self.queue.get(highest_priority_index).unwrap();
        Some(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue() {
        let mut queue = PriorityQueue::new();
        queue.enqueue('a', 3);
        queue.enqueue('b', 10);
        queue.enqueue('c', 1);
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn peek() {
        let mut queue = PriorityQueue::new();
        queue.enqueue('a', 3);
        queue.enqueue('b', 10);
        queue.enqueue('c', 1);
        assert_eq!(queue.peek(), Some(&'b'));
    }

    #[test]
    fn dequeue() {
        let mut queue = PriorityQueue::new();
        queue.enqueue('a', 3);
        queue.enqueue('b', 10);
        queue.enqueue('c', 1);

        let val = queue.dequeue();

        assert_eq!(queue.size(), 2);
        assert_eq!(val, Some('b'));
        assert_eq!(queue.peek(), Some(&'a'));
    }

    #[test]
    fn multiple_same_priority() {
        let mut queue = PriorityQueue::new();
        queue.enqueue('a', 3);
        queue.enqueue('b', 10);
        queue.enqueue('c', 1);
        queue.enqueue('d', 3);

        let val = queue.dequeue();
        assert_eq!(val, Some('b'));

        let val = queue.dequeue();
        assert_eq!(val, Some('a'));

        let val = queue.dequeue();
        assert_eq!(val, Some('d'));

        let val = queue.dequeue();
        assert_eq!(val, Some('c'));
    }

    #[test]
    fn enqueue_after_dequeue() {
        let mut queue = PriorityQueue::new();
        queue.enqueue('a', 3);
        queue.enqueue('b', 10);
        queue.enqueue('c', 1);
        queue.enqueue('d', 3);

        let val = queue.dequeue();
        assert_eq!(val, Some('b'));

        assert_eq!(queue.peek(), Some(&'a'));

        queue.enqueue('e', 5);
        queue.enqueue('f', 2);

        let val = queue.dequeue();
        assert_eq!(val, Some('e'));

        let val = queue.dequeue();
        assert_eq!(val, Some('a'));

        let val = queue.dequeue();
        assert_eq!(val, Some('d'));

        let val = queue.dequeue();
        assert_eq!(val, Some('f'));
    }

    #[test]
    fn empty() {
        let queue: PriorityQueue<char> = PriorityQueue::new();
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn dequeue_empty() {
        let mut queue: PriorityQueue<char> = PriorityQueue::new();

        let val = queue.dequeue();

        assert_eq!(val, None);
    }
}
