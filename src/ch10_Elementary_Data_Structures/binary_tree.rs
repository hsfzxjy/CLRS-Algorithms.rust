use std::cmp::{
    Eq,
    PartialEq,
};
use std::fmt;
use std::ops::{
    Deref,
    DerefMut,
};
use std::ptr::null_mut;

pub struct TreeNode<T> {
    index: usize,
    parent: *mut TreeNode<T>,
    childs: [*mut TreeNode<T>; 2],
    elem: T,
}

impl<T: fmt::Debug> fmt::Debug for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.elem.fmt(f)?;
        if self.childs[LEFT].is_null() && self.childs[RIGHT].is_null() {
            return Ok(());
        }
        write!(f, " {{ left: ")?;
        self.left()
            .and_then(|child| child.fmt(f).ok())
            .or_else(|| write!(f, "null").ok());
        write!(f, ", right: ")?;
        self.right()
            .and_then(|child| child.fmt(f).ok())
            .or_else(|| write!(f, "null").ok());
        write!(f, " }}")?;
        Ok(())
    }
}

const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = !0;

#[inline]
fn replace_node<T>(
    parent: &mut TreeNode<T>,
    index: usize,
    new_node: Option<Box<TreeNode<T>>>,
) -> Option<Box<TreeNode<T>>> {
    let new_node_ref;
    match new_node {
        None => new_node_ref = null_mut(),
        Some(mut node) => {
            node.parent = parent;
            node.index = index;
            new_node_ref = Box::into_raw(node);
        }
    }

    let old_ptr = parent.childs[index];
    parent.childs[index] = new_node_ref;

    if !old_ptr.is_null() {
        unsafe {
            let mut old_node = Box::from_raw(old_ptr);
            old_node.parent = null_mut();
            old_node.index = TOP;
            Some(old_node)
        }
    } else {
        None
    }
}

impl<T> TreeNode<T> {
    pub fn new(elem: T) -> TreeNode<T> {
        TreeNode {
            index: TOP,
            parent: null_mut(),
            childs: [null_mut(), null_mut()],
            elem: elem,
        }
    }

    #[inline]
    pub fn l(mut self, node: TreeNode<T>) -> Self {
        if !self.childs[LEFT].is_null() {
            panic!("Cannot set left child, as it's not empty.");
        }

        replace_node(&mut self, LEFT, Some(box node));
        self
    }

    #[inline]
    pub fn r(mut self: Self, node: TreeNode<T>) -> Self {
        if !self.childs[RIGHT].is_null() {
            panic!("Cannot set right child, as it's not empty.");
        }

        replace_node(&mut self, RIGHT, Some(box node));
        self
    }
}

impl<T> TreeNode<T> {
    #[inline]
    fn child(&self, index: usize) -> Option<Anchor<T>> {
        unsafe { self.childs[index].as_mut().map(|x| Anchor::new(x)) }
    }

    #[inline]
    fn child_unchecked(&self, index: usize) -> Anchor<T> {
        Anchor::new(self.childs[index])
    }
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn is_left(&self) -> bool {
        self.index == LEFT
    }

    #[inline]
    pub fn is_right(&self) -> bool {
        self.index == RIGHT
    }

    #[inline]
    pub fn is_root(&self) -> bool {
        self.index == TOP
    }

    #[inline]
    pub fn value(&self) -> &T {
        &self.elem
    }

    #[inline]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.elem
    }

    #[inline]
    pub fn left_raw(&self) -> *mut TreeNode<T> {
        self.childs[LEFT]
    }

    #[inline]
    pub fn right_raw(&self) -> *mut TreeNode<T> {
        self.childs[RIGHT]
    }

    #[inline]
    pub fn parent_raw(&self) -> *mut TreeNode<T> {
        self.parent
    }

    #[inline]
    pub unsafe fn left_unchecked(&self) -> Anchor<T> {
        self.child_unchecked(LEFT)
    }

    #[inline]
    pub unsafe fn right_unchecked(&self) -> Anchor<T> {
        self.child_unchecked(RIGHT)
    }

    #[inline]
    pub unsafe fn parent_unchecked(&self) -> Anchor<T> {
        Anchor::new(self.parent)
    }

    #[inline]
    pub fn left(&self) -> Option<Anchor<T>> {
        self.child(LEFT)
    }

    #[inline]
    pub fn right(&self) -> Option<Anchor<T>> {
        self.child(RIGHT)
    }

    #[inline]
    pub fn parent(&self) -> Option<Anchor<T>> {
        unsafe { self.parent.as_mut().map(|x| Anchor::new(x)) }
    }
}

impl<T> TreeNode<T> {
    pub fn replace_left(
        &mut self,
        node: Option<Box<TreeNode<T>>>,
    ) -> Option<Box<TreeNode<T>>> {
        replace_node(self, LEFT, node)
    }

    pub fn replace_right(
        &mut self,
        node: Option<Box<TreeNode<T>>>,
    ) -> Option<Box<TreeNode<T>>> {
        replace_node(self, RIGHT, node)
    }
}

impl<T: PartialEq> TreeNode<T> {
    fn matched_child(&self, other: &Self, index: usize) -> bool {
        unsafe {
            match (self.childs[index].as_ref(), other.childs[index].as_ref()) {
                (None, None) => true,
                (Some(x), Some(y)) => *x == *y,
                _ => false,
            }
        }
    }
}

impl<T> Drop for TreeNode<T> {
    fn drop(&mut self) {
        self.replace_left(None);
        self.replace_right(None);
    }
}

impl<T: PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem
            && self.matched_child(other, LEFT)
            && self.matched_child(other, RIGHT)
    }
}

impl<T: PartialEq> Eq for TreeNode<T> {}

#[derive(Copy, Clone)]
pub struct Anchor<T> {
    node: *mut TreeNode<T>,
}

impl<T: PartialEq> PartialEq for Anchor<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.node.is_null() || other.node.is_null() {
            self.node == other.node
        } else {
            unsafe { *self.node == *other.node }
        }
    }
}

impl<T: PartialEq> Eq for Anchor<T> {}

impl<T> Deref for Anchor<T> {
    type Target = TreeNode<T>;

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
    pub fn new(node: *mut TreeNode<T>) -> Anchor<T> {
        if node.is_null() {
            panic!("Cannot create anchor for null pointer.");
        }
        Anchor {
            node: node,
        }
    }

    #[inline]
    pub fn raw(&self) -> *mut TreeNode<T> {
        self.node
    }

    #[inline]
    pub fn detach(&self) -> Box<TreeNode<T>> {
        unsafe {
            if self.is_left() {
                self.parent_unchecked().replace_left(None).unwrap()
            } else if self.is_right() {
                self.parent_unchecked().replace_right(None).unwrap()
            } else {
                panic!("Cannot detach a dangling node")
            }
        }
    }
}

pub fn Node<T>(elem: T) -> TreeNode<T> {
    TreeNode::new(elem)
}

pub struct BinaryTree<T> {
    root: *mut TreeNode<T>,
}

#[derive(Copy, Clone)]
enum TraverseOrder {
    InOrder,
    PreOrder,
    PostOrder,
}

impl<T> BinaryTree<T> {
    pub fn new_empty() -> BinaryTree<T> {
        BinaryTree {
            root: null_mut(),
        }
    }

    pub fn new(node: TreeNode<T>) -> BinaryTree<T> {
        BinaryTree {
            root: Box::into_raw(box node),
        }
    }
}

impl<T> BinaryTree<T> {
    pub fn replace_root(
        &mut self,
        node: Option<Box<TreeNode<T>>>,
    ) -> Option<Box<TreeNode<T>>> {
        let old_node = if self.root.is_null() {
            None
        } else {
            unsafe { Some(Box::from_raw(self.root)) }
        };

        match node {
            None => self.root = null_mut(),
            Some(node) => self.root = Box::into_raw(node),
        }

        old_node
    }
}

impl<T> BinaryTree<T> {
    #[inline]
    pub fn root(&self) -> Option<Anchor<T>> {
        unsafe { self.root.as_mut().map(|x| Anchor::new(x)) }
    }

    #[inline]
    pub fn root_raw(&self) -> *mut TreeNode<T> {
        self.root
    }

    #[inline]
    pub unsafe fn root_unchecked(&self) -> Anchor<T> {
        Anchor::new(self.root)
    }
}

impl<T> BinaryTree<T> {
    fn traverse<F>(&self, f: &F, x: *const TreeNode<T>, order: TraverseOrder)
    where
        F: Fn(&T),
    {
        if x.is_null() {
            return;
        }

        let node = unsafe { x.as_ref().unwrap() };

        match order {
            TraverseOrder::InOrder => {
                f(node.value());
                self.traverse(f, node.left_raw(), order);
                self.traverse(f, node.right_raw(), order);
            }
            TraverseOrder::PreOrder => {
                self.traverse(f, node.left_raw(), order);
                f(node.value());
                self.traverse(f, node.right_raw(), order);
            }
            TraverseOrder::PostOrder => {
                self.traverse(f, node.left_raw(), order);
                self.traverse(f, node.right_raw(), order);
                f(node.value());
            }
        }
    }

    fn traverse_mut<F>(
        &mut self,
        f: &mut F,
        x: *mut TreeNode<T>,
        order: TraverseOrder,
    ) where
        F: FnMut(&mut T),
    {
        if x.is_null() {
            return;
        }

        let node = unsafe { x.as_mut().unwrap() };

        match order {
            TraverseOrder::InOrder => {
                f(node.value_mut());
                self.traverse_mut(f, node.left_raw(), order);
                self.traverse_mut(f, node.right_raw(), order);
            }
            TraverseOrder::PreOrder => {
                self.traverse_mut(f, node.left_raw(), order);
                f(node.value_mut());
                self.traverse_mut(f, node.right_raw(), order);
            }
            TraverseOrder::PostOrder => {
                self.traverse_mut(f, node.left_raw(), order);
                self.traverse_mut(f, node.right_raw(), order);
                f(node.value_mut());
            }
        }
    }

    pub fn in_order<F>(&mut self, f: F)
    where
        F: Fn(&T),
    {
        self.traverse(&f, self.root, TraverseOrder::InOrder);
    }

    pub fn pre_order<F>(&mut self, f: F)
    where
        F: Fn(&T),
    {
        self.traverse(&f, self.root, TraverseOrder::PreOrder);
    }

    pub fn post_order<F>(&mut self, f: F)
    where
        F: Fn(&T),
    {
        self.traverse(&f, self.root, TraverseOrder::PostOrder);
    }

    pub fn in_order_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        self.traverse_mut(&mut f, self.root, TraverseOrder::InOrder);
    }

    pub fn pre_order_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        self.traverse_mut(&mut f, self.root, TraverseOrder::PreOrder);
    }

    pub fn post_order_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        self.traverse_mut(&mut f, self.root, TraverseOrder::PostOrder);
    }
}

impl<T: PartialEq> PartialEq for BinaryTree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.root.is_null() || other.root.is_null() {
            self.root == other.root
        } else {
            unsafe { self.root_unchecked() == other.root_unchecked() }
        }
    }
}

impl<T: PartialEq> Eq for BinaryTree<T> {}

impl<T> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        if self.root.is_null() {
            return;
        }
        unsafe { Box::from_raw(self.root) };
    }
}

impl<T: fmt::Debug> fmt::Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.root.is_null() {
            write!(f, "null")
        } else {
            unsafe { self.root_unchecked().fmt(f) }
        }
    }
}

mod tests {

    #![allow(unused_imports)]
    use super::{
        BinaryTree,
        Node,
    };
    use testdrop::TestDrop;

    #[test]
    fn test_ctor_drop() {
        let td = TestDrop::new();
        let tree = BinaryTree::new(
            Node(td.new_item().1)
                .l(Node(td.new_item().1))
                .r(Node(td.new_item().1)),
        );
        drop(tree);
        assert_eq!(3, td.num_dropped_items());
    }

    #[should_panic]
    #[test]
    fn test_illegal_l() {
        let td = TestDrop::new();
        let _tree = BinaryTree::new(
            Node(td.new_item().1)
                .l(Node(td.new_item().1))
                .l(Node(td.new_item().1)),
        );
    }

    #[should_panic]
    #[test]
    fn test_illegal_r() {
        let td = TestDrop::new();
        let _tree = BinaryTree::new(
            Node(td.new_item().1)
                .r(Node(td.new_item().1))
                .r(Node(td.new_item().1)),
        );
    }

    #[test]
    fn test_replace_root() {
        let td = TestDrop::new();
        let mut tree = BinaryTree::new(
            Node(td.new_item().1)
                .l(Node(td.new_item().1))
                .r(Node(td.new_item().1)),
        );

        tree.replace_root(Some(box Node(td.new_item().1)));
        assert_eq!(3, td.num_dropped_items());
        drop(tree);
        assert_eq!(4, td.num_dropped_items());
    }

    #[test]
    fn test_replace_left() {
        let td = TestDrop::new();
        let tree = BinaryTree::new(
            Node(td.new_item().1)
                .l(Node(td.new_item().1))
                .r(Node(td.new_item().1)),
        );

        unsafe {
            tree.root_unchecked().replace_left(Some(
                box Node(td.new_item().1).l(Node(td.new_item().1)),
            ));
            assert_eq!(1, td.num_dropped_items());
        }

        drop(tree);
        assert_eq!(5, td.num_dropped_items());
    }

    #[test]
    fn test_replace_right() {
        let td = TestDrop::new();
        let tree = BinaryTree::new(
            Node(td.new_item().1)
                .l(Node(td.new_item().1))
                .r(Node(td.new_item().1)),
        );

        unsafe {
            tree.root_unchecked().replace_right(Some(
                box Node(td.new_item().1).l(Node(td.new_item().1)),
            ));
            assert_eq!(1, td.num_dropped_items());
        }

        drop(tree);
        assert_eq!(5, td.num_dropped_items());
    }

    #[test]
    fn test_eq() {
        let tree1 =
            BinaryTree::new(Node('a').l(Node('b').l(Node('c'))).r(Node('d')));
        let mut tree2 =
            BinaryTree::new(Node('a').l(Node('b').l(Node('c'))).r(Node('d')));
        assert_eq!(tree1, tree2);
        tree2.replace_root(Some(box Node('e')));
        assert_ne!(tree1, tree2);
    }

}
