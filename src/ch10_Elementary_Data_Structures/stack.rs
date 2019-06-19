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
        self.list.delete_back()
    }

    pub fn top_ref(&self) -> Option<&T> {
        self.list.back_ref()
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
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
        assert_eq!(stack.size(), 1);
        assert_eq!(*stack.top_ref().unwrap(), 'a');
    }

    #[test]
    fn test_pop() {
        let mut stack = super::Stack::new();
        stack.push('a');
        assert_eq!(stack.pop().unwrap(), 'a');
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_top() {
        let mut stack = super::Stack::new();
        stack.push('a');
        assert_eq!(*stack.top_ref().unwrap(), 'a');
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn test_top_mut() {
        let mut stack = super::Stack::new();
        stack.push('a');
        *stack.top_mut().unwrap() = 'b';
        assert_eq!(*stack.top_ref().unwrap(), 'b');
        assert_eq!(stack.size(), 1);
    }

}
