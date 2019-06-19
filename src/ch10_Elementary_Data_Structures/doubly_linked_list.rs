use std::cell::RefCell;
use std::ptr::NonNull;

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    data: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            next: None,
            prev: None,
        }
    }
}

pub struct Position<T> {
    list: RefCell<NonNull<DoublyLinkedList<T>>>,
    node: Link<T>,
}

impl<'a, T> Position<T> {
    fn new(list: &mut DoublyLinkedList<T>, node: &Link<T>) -> Position<T> {
        let cell = unsafe { RefCell::new(NonNull::new_unchecked(list)) };
        Position {
            list: cell,
            node: *node,
        }
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.node.as_ref().map(|n| unsafe { &n.as_ref().data })
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.node.as_mut().map(|n| unsafe { &mut n.as_mut().data })
    }

    pub fn next(&self) -> Option<Position<T>> {
        match self.node {
            None => None,
            Some(n) => unsafe {
                Some(Position {
                    list: self.list.clone(),
                    node: n.as_ref().next,
                })
            },
        }
    }

    pub fn prev(&self) -> Option<Position<T>> {
        match self.node {
            None => None,
            Some(n) => unsafe {
                Some(Position {
                    list: self.list.clone(),
                    node: n.as_ref().prev,
                })
            },
        }
    }

    pub fn delete(self) {
        unsafe {
            if self.is_valid() {
                self.list.borrow_mut().as_mut().delete(self.node);
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            if self.list.borrow_mut().as_mut().len() == 0 && self.node.is_none()
            {
                return true;
            }
            if self.list.borrow_mut().as_mut().len() > 0 && self.node.is_some()
            {
                return true;
            }
        }
        false
    }

    pub fn insert_before(&mut self, data: T) {
        unsafe {
            if self.is_valid() {
                self.list
                    .borrow_mut()
                    .as_mut()
                    .insert_before(&mut self.node, data);
            }
        }
    }
    pub fn insert_after(&mut self, data: T) {
        unsafe {
            if self.is_valid() {
                self.list
                    .borrow_mut()
                    .as_mut()
                    .insert_after(&mut self.node, data);
            }
        }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn insert_before(&mut self, link: &mut Link<T>, data: T) {
        match *link {
            None => self.insert_front(data),
            Some(mut node) => unsafe {
                let mut node = node.as_mut();
                if node.prev.is_none() {
                    self.insert_front(data);
                    return;
                }
                let mut new_node = box Node::new(data);
                new_node.prev = node.prev;
                new_node.next = link.clone();
                let new_node = Some(Box::into_raw_non_null(new_node));
                let mut prev = node.prev.unwrap();
                let mut prev = prev.as_mut();
                prev.next = new_node;
                node.prev = new_node;
                self.len += 1;
            },
        }
    }

    fn insert_after(&mut self, link: &mut Link<T>, data: T) {
        match *link {
            None => self.insert_back(data),
            Some(mut node) => unsafe {
                let mut node = node.as_mut();
                if node.next.is_none() {
                    self.insert_back(data);
                    return;
                }
                let mut new_node = box Node::new(data);
                new_node.prev = *link;
                new_node.next = node.next;
                let new_node = Some(Box::into_raw_non_null(new_node));
                let mut next = node.next.unwrap();
                let mut next = next.as_mut();
                next.prev = new_node;
                node.next = new_node;
                self.len += 1;
            },
        }
    }

    fn delete(&mut self, link: Link<T>) {
        link.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());

            match (node.prev, node.next) {
                (None, _) => {
                    self.delete_front();
                }
                (_, None) => {
                    self.delete_back();
                }
                _ => {
                    node.prev.unwrap().as_mut().next = node.next;
                    node.next.unwrap().as_mut().prev = node.prev;
                    self.len -= 1;
                }
            }
        });
    }

    pub fn insert_front(&mut self, data: T) {
        let mut node = box Node::new(data);
        node.next = self.head;

        let node = Some(Box::into_raw_non_null(node));

        match self.head {
            None => self.tail = node,
            Some(ref mut ptr) => unsafe { (*NonNull::as_mut(ptr)).prev = node },
        }

        self.len += 1;
        self.head = node;
    }

    pub fn insert_back(&mut self, data: T) {
        let mut node = box Node::new(data);
        node.prev = self.tail;

        let node = Some(Box::into_raw_non_null(node));

        match self.tail {
            None => self.head = node,
            Some(ref mut ptr) => unsafe { (*NonNull::as_mut(ptr)).next = node },
        }

        self.len += 1;
        self.tail = node;
    }

    pub fn delete_front(&mut self) -> Option<T> {
        self.head.map(|head| unsafe {
            let head = Box::from_raw(head.as_ptr());
            self.head = head.next;

            match head.next {
                None => self.tail = None,
                Some(mut n) => (*n.as_mut()).prev = None,
            }

            self.len -= 1;
            head.data
        })
    }

    pub fn delete_back(&mut self) -> Option<T> {
        self.tail.map(|tail| unsafe {
            let tail = Box::from_raw(tail.as_ptr());
            self.tail = tail.prev;

            match tail.prev {
                None => self.head = None,
                Some(mut n) => (*n.as_mut()).next = None,
            }

            self.len -= 1;
            tail.data
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&mut self) -> Position<T> {
        Position::new(self, &mut self.head.clone())
    }

    pub fn back(&mut self) -> Position<T> {
        Position::new(self, &mut self.tail.clone())
    }

    pub fn iter(&self) -> DoublyLinkedListIterator<T> {
        DoublyLinkedListIterator::new(&self)
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.delete_back() {}
    }
}

pub struct DoublyLinkedListIterator<'a, T> {
    list: &'a DoublyLinkedList<T>,
    node: &'a Link<T>,
}

impl<'a, T> DoublyLinkedListIterator<'a, T> {
    fn new(lst: &'a DoublyLinkedList<T>) -> DoublyLinkedListIterator<T> {
        DoublyLinkedListIterator {
            list: lst,
            node: &lst.head,
        }
    }
}

impl<'a, T> Iterator for DoublyLinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.node.as_ref().map(|ptr| unsafe {
            let result = &ptr.as_ref().data;
            self.node = &ptr.as_ref().next;
            result
        })
    }
}

#[test]
fn test_insert_front() {
    let mut lst = DoublyLinkedList::new();
    lst.insert_front(3i32);
    assert_eq!(*lst.front().as_ref().unwrap(), 3);
    assert_eq!(*lst.back().as_ref().unwrap(), 3);
    assert_eq!(lst.len(), 1);
    lst.insert_front(4i32);
    assert_eq!(*lst.front().as_ref().unwrap(), 4);
    assert_eq!(*lst.back().as_ref().unwrap(), 3);
    assert_eq!(lst.len(), 2);
}

#[test]
fn test_insert_back() {
    let mut lst = DoublyLinkedList::new();
    lst.insert_back(3i32);
    assert_eq!(*lst.front().as_ref().unwrap(), 3);
    assert_eq!(*lst.back().as_ref().unwrap(), 3);
    assert_eq!(lst.len(), 1);
    lst.insert_back(4i32);
    assert_eq!(*lst.front().as_ref().unwrap(), 3);
    assert_eq!(*lst.back().as_ref().unwrap(), 4);
    assert_eq!(lst.len(), 2);
}

#[test]
fn test_delete_front() {
    let mut lst = DoublyLinkedList::new();
    for i in 0..10 {
        lst.insert_back(i);
    }

    for i in 0..10 {
        assert_eq!(lst.delete_front().unwrap(), i);
    }
}

#[test]
fn test_delete_back() {
    let mut lst = DoublyLinkedList::new();
    for i in 0..10 {
        lst.insert_back(i);
    }

    for i in (0..10).rev() {
        assert_eq!(lst.delete_back().unwrap(), i);
    }
}

#[test]
fn test_front_mut() {
    let mut lst = DoublyLinkedList::new();
    lst.insert_back(1i32);
    *lst.front().as_mut().unwrap() = 2;
    assert_eq!(*lst.front().as_ref().unwrap(), 2);
}

#[test]
fn test_back_mut() {
    let mut lst = DoublyLinkedList::new();
    lst.insert_back(0i32);
    lst.insert_back(1i32);
    *lst.back().as_mut().unwrap() = 2;
    assert_eq!(*lst.back().as_ref().unwrap(), 2);
    lst.back().prev().unwrap().next().unwrap().insert_after(10);
    assert_eq!(*lst.back().as_ref().unwrap(), 10);
}
