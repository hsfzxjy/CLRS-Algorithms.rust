use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::null_mut;
pub struct Node<T> {
    elem: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node {
            elem: elem,
            prev: null_mut(),
            next: null_mut(),
        }
    }

    #[inline]
    pub fn prev_raw(&self) -> *mut Node<T> {
        self.prev
    }

    #[inline]
    pub fn next_raw(&self) -> *mut Node<T> {
        self.next
    }

    #[inline]
    pub fn value_ref(&self) -> &T {
        &self.elem
    }

    #[inline]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.elem
    }

    #[inline]
    pub fn into_value(self) -> T {
        self.elem
    }
}

impl<T: Copy> Node<T> {
    #[inline]
    pub fn value(&self) -> T {
        self.elem
    }
}

pub struct Anchor<T> {
    node: *mut Node<T>,
    list: *mut DoublyLinkedList<T>,
}

impl<T> Copy for Anchor<T> {}

impl<T> Clone for Anchor<T> {
    fn clone(&self) -> Self {
        Anchor::new(self.node, self.list)
    }
}

impl<T> Deref for Anchor<T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.node.as_ref().unwrap() }
    }
}

impl<T> DerefMut for Anchor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.node.as_mut().unwrap() }
    }
}

impl<T> Anchor<T> {
    #[inline]
    pub unsafe fn next_unchecked(&self) -> Anchor<T> {
        Anchor::new(self.next, self.list)
    }

    #[inline]
    pub fn next(&self) -> Option<Anchor<T>> {
        unsafe { self.next.as_mut().map(|x| Anchor::new(x, self.list)) }
    }

    #[inline]
    pub unsafe fn prev_unchecked(&self) -> Anchor<T> {
        Anchor::new(self.prev, self.list)
    }

    #[inline]
    pub fn prev(&self) -> Option<Anchor<T>> {
        unsafe { self.prev.as_mut().map(|x| Anchor::new(x, self.list)) }
    }

    #[inline]
    unsafe fn list_unchecked(&self) -> &mut DoublyLinkedList<T> {
        self.list.as_mut().unwrap()
    }

    #[inline]
    pub fn is_dangling(&self) -> bool {
        self.list.is_null()
    }

    #[inline]
    pub fn is_head(&self) -> bool {
        !self.is_dangling() && self.prev.is_null()
    }

    #[inline]
    pub fn is_tail(&self) -> bool {
        !self.is_dangling() && self.next.is_null()
    }
}

impl<T> Anchor<T> {
    pub fn new(
        node: *mut Node<T>,
        list: *mut DoublyLinkedList<T>,
    ) -> Anchor<T> {
        if node.is_null() {
            panic!("Cannot create anchor for null pointer.")
        }
        Anchor {
            node: node,
            list: list,
        }
    }

    #[inline]
    pub fn raw(&self) -> *mut Node<T> {
        self.node
    }

    #[inline]
    pub fn detach(self) -> Box<Node<T>> {
        if self.is_dangling() {
            panic!("Cannot detach a dangling node.");
        }

        unsafe {
            let mut current = Box::from_raw(self.node);

            if self.is_head() {
                self.list_unchecked().head = self.next;
            } else {
                self.prev_unchecked().next = self.next;
            }

            if self.is_tail() {
                self.list_unchecked().tail = self.prev;
            } else {
                self.next_unchecked().prev = self.prev;
            }

            current.prev = null_mut();
            current.next = null_mut();

            self.list_unchecked().len -= 1;
            current
        }
    }

    #[inline]
    pub fn delete(self) {
        self.detach();
    }

    #[inline]
    pub fn insert_before(&mut self, elem: T) -> Anchor<T> {
        if self.is_dangling() {
            panic!("Cannot insert before a dangling node.")
        }

        let mut new_node = box Node::new(elem);
        new_node.prev = self.prev;
        new_node.next = self.raw();
        let new_node_ptr = Box::into_raw(new_node);
        unsafe {
            if self.is_head() {
                self.list_unchecked().head = new_node_ptr;
            } else {
                self.prev_unchecked().next = new_node_ptr;
            }
        }
        self.prev = new_node_ptr;
        unsafe {
            self.list_unchecked().len += 1;
        }

        Anchor::new(new_node_ptr, self.list)
    }

    #[inline]
    pub fn insert_after(&mut self, elem: T) -> Anchor<T> {
        if self.is_dangling() {
            panic!("Cannot insert after a dangling node.")
        }

        let mut new_node = box Node::new(elem);
        new_node.next = self.next;
        new_node.prev = self.raw();
        let new_node_ptr = Box::into_raw(new_node);
        unsafe {
            if self.is_tail() {
                self.list_unchecked().tail = new_node_ptr;
            } else {
                self.next_unchecked().prev = new_node_ptr;
            }
        }
        self.next = new_node_ptr;

        unsafe {
            self.list_unchecked().len += 1;
        }

        Anchor::new(new_node_ptr, self.list)
    }
}

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn front_ref(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|x| &x.elem) }
    }

    pub fn back_ref(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|x| &x.elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|x| &mut x.elem) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|x| &mut x.elem) }
    }
}
impl<T> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn head_raw(&self) -> *mut Node<T> {
        self.head
    }

    pub unsafe fn head_unchecked(&self) -> Anchor<T> {
        Anchor::new(self.head, self._transmute())
    }

    pub fn head(&self) -> Option<Anchor<T>> {
        unsafe { self.head.as_mut().map(|x| Anchor::new(x, self._transmute())) }
    }

    pub fn tail_raw(&self) -> *mut Node<T> {
        self.tail
    }

    pub unsafe fn tail_unchecked(&self) -> Anchor<T> {
        Anchor::new(self.tail, self._transmute())
    }

    pub fn tail(&self) -> Option<Anchor<T>> {
        unsafe { self.tail.as_mut().map(|x| Anchor::new(x, self._transmute())) }
    }
}

impl<T> DoublyLinkedList<T> {
    #[inline]
    unsafe fn _transmute(&self) -> *mut Self {
        mem::transmute(self)
    }

    fn insert_first_node(&mut self, elem: T) -> Anchor<T> {
        let node_ptr = Box::into_raw(box Node::new(elem));
        self.head = node_ptr;
        self.tail = node_ptr;
        self.len += 1;
        Anchor::new(node_ptr, self)
    }

    pub fn insert_front(&mut self, elem: T) -> Anchor<T> {
        if self.is_empty() {
            self.insert_first_node(elem)
        } else {
            unsafe { self.head_unchecked().insert_before(elem) }
        }
    }

    pub fn insert_back(&mut self, elem: T) -> Anchor<T> {
        if self.is_empty() {
            self.insert_first_node(elem)
        } else {
            unsafe { self.tail_unchecked().insert_after(elem) }
        }
    }

    pub fn detach_front(&mut self) -> Option<Box<Node<T>>> {
        self.head().map(|x| x.detach())
    }

    pub fn detach_back(&mut self) -> Option<Box<Node<T>>> {
        self.tail().map(|x| x.detach())
    }

    pub fn iter(&self) -> DoublyLinkedListIterator<T> {
        DoublyLinkedListIterator::new(self, IteratorDirection::Backward)
    }

    pub fn iter_rev(&self) -> DoublyLinkedListIterator<T> {
        DoublyLinkedListIterator::new(self, IteratorDirection::Forward)
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.detach_back() {}
    }
}

enum IteratorDirection {
    Forward,
    Backward,
}

pub struct DoublyLinkedListIterator<'a, T> {
    list: &'a DoublyLinkedList<T>,
    anchor: Option<Anchor<T>>,
    direction: IteratorDirection,
}

impl<'a, T> DoublyLinkedListIterator<'a, T> {
    fn new(
        lst: &'a DoublyLinkedList<T>,
        direction: IteratorDirection,
    ) -> DoublyLinkedListIterator<T> {
        DoublyLinkedListIterator {
            list: lst,
            anchor: match direction {
                IteratorDirection::Backward => lst.head(),
                IteratorDirection::Forward => lst.tail(),
            },
            direction,
        }
    }
}

impl<'a, T> Iterator for DoublyLinkedListIterator<'a, T> {
    type Item = Anchor<T>;

    fn next(&mut self) -> Option<Anchor<T>> {
        let next_anchor =
            self.anchor.as_ref().and_then(|anchor| match self.direction {
                IteratorDirection::Backward => anchor.next(),
                IteratorDirection::Forward => anchor.prev(),
            });
        self.anchor = next_anchor;
        next_anchor
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::DoublyLinkedList;
    use testdrop::TestDrop;

    #[test]
    fn test_drop() {
        let td = TestDrop::new();
        {
            let mut lst = DoublyLinkedList::new();
            lst.insert_front(td.new_item().1);
            lst.insert_front(td.new_item().1);
        }
        assert_eq!(2, td.num_dropped_items());
    }

    #[test]
    fn test_insert_front() {
        let mut lst = DoublyLinkedList::new();
        lst.insert_front(2i32);
        lst.insert_front(1i32);
        unsafe {
            assert_eq!(lst.len(), 2);

            assert_eq!(lst.head_unchecked().value(), 1);
            assert_eq!(lst.head_unchecked().next_unchecked().value(), 2);
            assert!(lst.head_unchecked().next_unchecked().next().is_none());

            assert_eq!(lst.tail_unchecked().value(), 2);
            assert_eq!(lst.tail_unchecked().prev_unchecked().value(), 1);
            assert!(lst.tail_unchecked().prev_unchecked().prev().is_none());

            assert_eq!(
                lst.tail_unchecked().prev_unchecked().next_unchecked().value(),
                2
            );
        }
    }

    #[test]
    fn test_insert_back() {
        let mut lst = DoublyLinkedList::new();
        lst.insert_back(1i32);
        lst.insert_back(2i32);
        unsafe {
            assert_eq!(lst.len(), 2);

            assert_eq!(lst.head_unchecked().value(), 1);
            assert_eq!(lst.head_unchecked().next_unchecked().value(), 2);
            assert!(lst.head_unchecked().next_unchecked().next().is_none());

            assert_eq!(lst.tail_unchecked().value(), 2);
            assert_eq!(lst.tail_unchecked().prev_unchecked().value(), 1);
            assert!(lst.tail_unchecked().prev_unchecked().prev().is_none());

            assert_eq!(
                lst.tail_unchecked().prev_unchecked().next_unchecked().value(),
                2
            );
        }
    }

    #[test]
    fn test_detach_front() {
        let mut lst = DoublyLinkedList::new();
        lst.insert_back(1i32);
        lst.insert_back(2i32);

        assert_eq!(lst.detach_front().unwrap().value(), 1i32);

        unsafe {
            assert_eq!(lst.len(), 1);

            assert_eq!(lst.head_unchecked().value(), 2);
            assert!(lst.head_unchecked().next().is_none());
        }
    }
}
