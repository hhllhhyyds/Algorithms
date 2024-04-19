use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    #[allow(dead_code)]
    parent: Weak<RefCell<TreeNode<T>>>,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
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

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
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
