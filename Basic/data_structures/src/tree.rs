use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

use crate::queue::Queue;

#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub parent: Weak<RefCell<TreeNode<T>>>,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

impl<T> TreeNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            value: x,
            parent: Weak::new(),
            left: BinaryTree::default(),
            right: BinaryTree::default(),
        }
    }

    pub fn new_leaf(x: T, parent: Weak<RefCell<TreeNode<T>>>) -> Self {
        Self {
            value: x,
            parent,
            left: BinaryTree::default(),
            right: BinaryTree::default(),
        }
    }

    pub fn to_root(self) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(self)))
    }
}

impl<T: PartialEq + PartialOrd> TreeNode<T> {
    pub fn left_to(&self, node: &TreeNode<T>) -> bool {
        self.value.lt(&node.value)
    }
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        BinaryTree { root: None }
    }
}

impl<T> BinaryTree<T> {
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn clean(&mut self) {
        self.root.take();
    }

    pub fn iter(&self) -> Iter<T> {
        fn enqueue<T>(queue: &mut Vec<Rc<RefCell<TreeNode<T>>>>, tree: &BinaryTree<T>) {
            if !tree.is_empty() {
                unsafe {
                    enqueue(
                        queue,
                        &tree.root.as_ref().unwrap_unchecked().as_ref().borrow().left,
                    );
                    Queue::enqueue(queue, tree.root.as_ref().unwrap_unchecked().clone());
                    enqueue(
                        queue,
                        &tree
                            .root
                            .as_ref()
                            .unwrap_unchecked()
                            .as_ref()
                            .borrow()
                            .right,
                    );
                }
            }
        }
        let mut queue = Vec::new();
        enqueue(&mut queue, self);

        Iter(queue)
    }
}

impl<T: PartialEq + PartialOrd> BinaryTree<T> {
    pub fn insert(&mut self, x: T) {
        if self.is_empty() {
            self.root = TreeNode::new(x).to_root();
        } else {
            unsafe {
                let mut cursor = self.root.as_ref().unwrap_unchecked().clone();
                loop {
                    if x < cursor.borrow().value {
                        if cursor.borrow().left.is_empty() {
                            cursor.borrow_mut().left.root =
                                TreeNode::new_leaf(x, Rc::downgrade(&cursor)).to_root();
                            return;
                        } else {
                            let next = cursor
                                .borrow()
                                .left
                                .root
                                .as_ref()
                                .unwrap_unchecked()
                                .clone();
                            cursor = next;
                        }
                    } else if cursor.borrow().right.is_empty() {
                        cursor.borrow_mut().right.root =
                            TreeNode::new_leaf(x, Rc::downgrade(&cursor)).to_root();
                        return;
                    } else {
                        let next = cursor
                            .borrow()
                            .right
                            .root
                            .as_ref()
                            .unwrap_unchecked()
                            .clone();
                        cursor = next;
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, node: Rc<RefCell<TreeNode<T>>>) {
        if node.borrow().left.is_empty() {
            if node.borrow().right.is_empty() {
                if node.borrow().parent.upgrade().is_some() {
                    let parent = node.borrow().parent.upgrade().unwrap();
                    if node.borrow().left_to(&parent.borrow()) {
                        parent.borrow_mut().left.clean();
                    } else {
                        parent.borrow_mut().right.clean();
                    }
                } else {
                    self.clean();
                }
            } else {
                let right_node = node.borrow_mut().right.root.take().unwrap();
                if let Some(parent) = node.borrow().parent.upgrade() {
                    right_node.borrow_mut().parent = Rc::downgrade(&parent);
                    if node.borrow().left_to(&parent.borrow()) {
                        parent.borrow_mut().left.root = Some(right_node);
                    } else {
                        parent.borrow_mut().right.root = Some(right_node);
                    }
                } else {
                    right_node.borrow_mut().parent = Weak::new();
                    self.root = Some(right_node);
                }
            }
        } else if node.borrow().right.is_empty() {
            let left_node = node.borrow_mut().left.root.take().unwrap();
            if let Some(parent) = node.borrow().parent.upgrade() {
                left_node.borrow_mut().parent = Rc::downgrade(&parent);
                if node.borrow().left_to(&parent.borrow()) {
                    parent.borrow_mut().left.root = Some(left_node);
                } else {
                    parent.borrow_mut().right.root = Some(left_node);
                }
            } else {
                left_node.borrow_mut().parent = Weak::new();
                self.root = Some(left_node);
            }
        } else {
            let mut right_node = node.borrow_mut().right.root.take();
            right_node.as_mut().unwrap().borrow_mut().parent = Weak::new();
            let mut right_tree = BinaryTree { root: right_node };
            let right_min = right_tree.min().unwrap();
            right_tree.remove(right_min.clone());
            node.borrow_mut()
                .left
                .root
                .as_mut()
                .unwrap()
                .borrow_mut()
                .parent = Rc::downgrade(&right_min);
            right_min.borrow_mut().left.root = node.borrow_mut().left.root.take();
            if let Some(root) = right_tree.root.as_mut() {
                root.borrow_mut().parent = Rc::downgrade(&right_min);
            }
            right_min.borrow_mut().right.root = right_tree.root.take();
            if let Some(parent) = node.borrow().parent.upgrade() {
                right_min.borrow_mut().parent = Rc::downgrade(&parent);
                if node.borrow().left_to(&parent.borrow()) {
                    parent.borrow_mut().left.root = Some(right_min);
                } else {
                    parent.borrow_mut().right.root = Some(right_min);
                }
            } else {
                right_min.borrow_mut().parent = Weak::new();
                self.root = Some(right_min);
            }
        }

        node.borrow_mut().parent = Weak::new();
    }

    pub fn find(&self, x: &T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if self.is_empty() {
            None
        } else {
            unsafe {
                let cursor = self.root.as_ref().unwrap_unchecked();
                if cursor.borrow().value.eq(x) {
                    Some(cursor.clone())
                } else {
                    let left_ret = cursor.borrow().left.find(x);
                    let right_ret = cursor.borrow().right.find(x);
                    if left_ret.is_some() {
                        left_ret
                    } else {
                        right_ret
                    }
                }
            }
        }
    }

    pub fn min(&self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if self.is_empty() {
            None
        } else {
            unsafe {
                let mut cursor = self.root.as_ref().unwrap_unchecked().clone();
                loop {
                    if cursor.borrow().left.is_empty() {
                        return Some(cursor.clone());
                    } else {
                        let next = cursor
                            .as_ref()
                            .borrow()
                            .left
                            .root
                            .as_ref()
                            .unwrap_unchecked()
                            .clone();
                        cursor = next;
                    }
                }
            }
        }
    }

    pub fn max(&self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if self.is_empty() {
            None
        } else {
            unsafe {
                let mut cursor = self.root.as_ref().unwrap_unchecked().clone();
                loop {
                    if cursor.borrow().right.is_empty() {
                        return Some(cursor.clone());
                    } else {
                        let next = cursor
                            .as_ref()
                            .borrow()
                            .right
                            .root
                            .as_ref()
                            .unwrap_unchecked()
                            .clone();
                        cursor = next;
                    }
                }
            }
        }
    }
}

fn fmt_with_count<T: Display>(
    tree: &BinaryTree<T>,
    f: &mut std::fmt::Formatter<'_>,
    count: usize,
) -> std::fmt::Result {
    fn write_space(i: usize, div: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if i % div == 0 {
            write!(f, "|")?;
        } else {
            write!(f, " ")?;
        }

        Ok(())
    }

    fn write_line(
        space_count: usize,
        arm: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        writeln!(f)?;
        for i in 0..space_count {
            write_space(i, arm.len(), f)?;
        }
        writeln!(f, "|")?;
        for i in 0..space_count {
            write_space(i, arm.len(), f)?;
        }
        write!(f, "{arm}")?;

        Ok(())
    }

    if let Some(node) = tree.root.as_ref() {
        let left_arm = "--l--";
        let right_arm = "--r--";
        let arm_len = left_arm.len();
        let space_count = count * arm_len;
        node.borrow().value.fmt(f)?;
        if !node.borrow().left.is_empty() {
            write_line(space_count, left_arm, f)?;
            fmt_with_count(&node.borrow().left, f, count + 1)?;
        }
        if !node.borrow().right.is_empty() {
            write_line(space_count, right_arm, f)?;
            fmt_with_count(&node.borrow().right, f, count + 1)?;
        }
    };

    Ok(())
}

impl<T: Display> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_with_count(self, f, 0)
    }
}

pub struct Iter<T>(Vec<Rc<RefCell<TreeNode<T>>>>);

impl<T> Iterator for Iter<T> {
    type Item = Rc<RefCell<TreeNode<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        Queue::dequeue(&mut self.0)
    }
}
