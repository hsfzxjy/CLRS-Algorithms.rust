use crate::ch10_Elementary_Data_Structures::binary_tree::{
    Anchor,
    BinaryTree,
    Node,
    TreeNode,
};
use std::ptr::null_mut;

pub struct BST<T: PartialOrd + Copy> {
    tree: BinaryTree<T>,
}

impl<T: PartialOrd + Copy> BST<T> {
    pub fn new() -> BST<T> {
        BST {
            tree: BinaryTree::new_empty(),
        }
    }

    pub fn insert(&mut self, elem: T) {
        unsafe {
            let mut y = null_mut();
            let mut x = self.tree.root_raw();
            let z = box Node(elem);

            while !x.is_null() {
                y = x;
                x = if elem < *(*x).value() {
                    (*x).left_raw()
                } else {
                    (*x).right_raw()
                }
            }

            if y.is_null() {
                self.tree.replace_root(Some(z));
                return;
            }

            if elem < *(*y).value() {
                (*y).replace_left(Some(z));
            } else {
                (*y).replace_right(Some(z));
            }
        }
    }

    unsafe fn transplant(
        &mut self,
        u: Anchor<T>,
        v: Option<Box<TreeNode<T>>>,
    ) -> Option<Box<TreeNode<T>>> {
        if u.is_root() {
            self.tree.replace_root(v)
        } else if u.is_left() {
            u.parent_unchecked().replace_left(v)
        } else {
            u.parent_unchecked().replace_right(v)
        }
    }

    fn delete(&mut self, mut z: Anchor<T>) {
        unsafe {
            if z.left_raw().is_null() {
                let z_right = z.replace_right(None);
                self.transplant(z, z_right);
            } else if z.right_raw().is_null() {
                let z_left = z.replace_left(None);
                self.transplant(z, z_left);
            } else {
                let mut y = {
                    let mut z_min = self.minimum_from(z.right_unchecked());
                    if z_min.parent_unchecked() != z {
                        let z_min_right = z_min.replace_right(None);
                        let mut z_min = z_min
                            .parent_unchecked()
                            .replace_left(z_min_right)
                            .unwrap();
                        let z_right = z.right_unchecked().detach();
                        z_min.replace_right(Some(z_right));
                        z_min
                    } else {
                        z_min.detach()
                    }
                };
                let z_left = z.replace_left(None);
                y.replace_left(z_left);
                self.transplant(z, Some(y));
            }
        }
    }

    fn minimum_from(&self, x: Anchor<T>) -> Anchor<T> {
        let mut x = x.raw();
        let mut y = x;
        while !x.is_null() {
            y = x;
            x = unsafe { (*x).left_raw() };
        }
        Anchor::new(y)
    }

    fn minimum(&self) -> Option<Anchor<T>> {
        self.tree.root().map(|x| self.minimum_from(x))
    }

    fn maximum_from(&self, x: Anchor<T>) -> Anchor<T> {
        let mut x = x.raw();
        let mut y = x;
        while !x.is_null() {
            y = x;
            x = unsafe { (*x).right_raw() };
        }
        Anchor::new(y)
    }

    fn maximum(&self) -> Option<Anchor<T>> {
        self.tree.root().map(|x| self.maximum_from(x))
    }
}

mod tests {

    #![allow(unused_imports)]
    use super::BST;
    use crate::ch10_Elementary_Data_Structures::binary_tree::*;

    #[rustfmt_skip]
    #[test]
    fn test_insert() {
        let mut tree = BST::new();
        tree.insert(2f64);
        tree.insert(1.);
        tree.insert(3.);
        tree.insert(1.5);
        tree.insert(2.5);
        tree.insert(2.4);
        tree.insert(3.1);

        assert_eq!(
            tree.tree,
            BinaryTree::new(
                Node(2f64)
                .l(
                    Node(1.)
                    .r(
                        Node(1.5)
                    )
                )
                .r(
                    Node(3.)
                    .l(
                        Node(2.5)
                        .l(
                            Node(2.4)
                        )
                    )
                    .r(
                        Node(3.1)
                    )    
                )
            )
        );
    }

    #[rustfmt_skip]
    #[test]
    fn test_delete() {
        let mut tree = BST::new();
        tree.insert(2f64);
        tree.insert(1.);
        tree.insert(3.);
        tree.insert(1.5);
        tree.insert(2.5);
        tree.insert(2.4);
        tree.insert(2.45);
        tree.insert(3.1);


        tree.delete(unsafe { tree.tree.root_unchecked() });
        assert_eq!(
            tree.tree,
            BinaryTree::new(
                Node(2.4f64)
                .l(
                    Node(1.)
                    .r(
                        Node(1.5)
                    )
                )
                .r(
                    Node(3.)
                    .l(
                        Node(2.5)
                        .l(
                            Node(2.45)
                        )
                    )
                    .r(
                        Node(3.1)
                    )    
                )
            )
        );
        tree.delete(unsafe { tree.tree.root_unchecked().right_unchecked() });
        assert_eq!(
            tree.tree,
            BinaryTree::new(
                Node(2.4f64)
                .l(
                    Node(1.)
                    .r(
                        Node(1.5)
                    )
                )
                .r(
                    Node(3.1)
                    .l(
                        Node(2.5)
                        .l(
                            Node(2.45)
                        )
                    )
                )
            )
        );
    }

}
