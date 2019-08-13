use super::doubly_linked_list::DoublyLinkedList;

struct Stack<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            list: DoublyLinkedList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn push(&mut self, elem: T) {
        self.list.insert_back(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.detach_back().map(|x| x.into_value())
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.back_ref()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.list.back_mut()
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }
}

mod tests {
    #[test]
    fn test_stack_is_empty() {
        let stack: super::Stack<i32> = super::Stack::new();
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn test_push() {
        let mut stack = super::Stack::new();
        stack.push('a');
        stack.push('b');
        assert_eq!(stack.size(), 2);
        assert_eq!(*stack.peek().unwrap(), 'b');
    }

    #[test]
    fn test_pop() {
        let mut stack = super::Stack::new();
        stack.push('a');
        stack.push('b');
        assert_eq!(stack.pop().unwrap(), 'b');
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn test_peek() {
        let mut stack = super::Stack::new();
        stack.push('a');
        assert_eq!(*stack.peek().unwrap(), 'a');
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn test_peek_mut() {
        let mut stack = super::Stack::new();
        stack.push('a');
        *stack.peek_mut().unwrap() = 'b';
        assert_eq!(*stack.peek().unwrap(), 'b');
        assert_eq!(stack.size(), 1);
    }
}
