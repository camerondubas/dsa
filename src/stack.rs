pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { top: None }
    }

    pub fn push(&mut self, data: T) {
        let next = self.top.take();

        let node = Node { data, next };
        let new_box = Box::new(node);

        self.top = Some(new_box);
    }

    pub fn pop(&mut self) -> Option<T> {
        let item = self.top.take()?;

        self.top = item.next;

        Some(item.data)
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(node) => Some(&node.data),
            None => None,
        }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut stack = Stack::new();
        stack.push('a');
        stack.push('b');
        assert_eq!(stack.peek(), Some(&'b'));
        stack.push('c');
        assert_eq!(stack.peek(), Some(&'c'));
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        stack.push('a');
        stack.push('b');
        stack.push('c');
        assert_eq!(stack.pop(), Some('c'));
        assert_eq!(stack.peek(), Some(&'b'));
        assert_eq!(stack.pop(), Some('b'));
        assert_eq!(stack.peek(), Some(&'a'));
    }

    #[test]
    fn empty() {
        let mut stack: Stack<char> = Stack::new();
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
    }
}
