use super::doubly_linked_list::DoublyLinkedList;

struct Queue<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            list: DoublyLinkedList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn enque(&mut self, elem: T) {
        self.list.insert_front(elem);
    }

    pub fn deque(&mut self) -> Option<T> {
        self.list.detach_back().map(|x| x.into_value())
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.back_ref()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.list.back_mut()
    }
}

mod tests {
    #[test]
    fn test_queue_is_empty() {
        let queue: super::Queue<i32> = super::Queue::new();
        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn test_enque() {
        let mut queue = super::Queue::new();
        queue.enque('a');
        queue.enque('b');
        assert_eq!(queue.size(), 2);
        assert_eq!(*queue.peek().unwrap(), 'a');
    }

    #[test]
    fn test_deque() {
        let mut queue = super::Queue::new();
        queue.enque('a');
        queue.enque('b');
        assert_eq!(queue.deque().unwrap(), 'a');
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn test_peek() {
        let mut queue = super::Queue::new();
        queue.enque('a');
        queue.enque('b');
        assert_eq!(*queue.peek().unwrap(), 'a');
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn test_peek_mut() {
        let mut queue = super::Queue::new();
        queue.enque('a');
        *queue.peek_mut().unwrap() = 'b';
        assert_eq!(*queue.peek().unwrap(), 'b');
        assert_eq!(queue.size(), 1);
    }
}
