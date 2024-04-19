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
        self.insert_under_parent(x, Weak::new());
    }

    fn insert_under_parent(&mut self, x: T, parent: Weak<RefCell<TreeNode<T>>>) {
        // TODO: use loop instead of recursion
        if self.is_empty() {
            self.root = Some(Rc::new(RefCell::new(TreeNode {
                value: x,
                parent,
                left: BinaryTree::default(),
                right: BinaryTree::default(),
            })));
        } else {
            unsafe {
                let parent = Rc::downgrade(self.root.as_ref().unwrap_unchecked());
                if x < self.root.as_ref().unwrap_unchecked().borrow().value {
                    self.root
                        .as_mut()
                        .unwrap_unchecked()
                        .borrow_mut()
                        .left
                        .insert_under_parent(x, parent);
                } else {
                    self.root
                        .as_mut()
                        .unwrap_unchecked()
                        .borrow_mut()
                        .right
                        .insert_under_parent(x, parent);
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
