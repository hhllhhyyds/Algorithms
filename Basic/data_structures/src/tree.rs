use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub parent: Weak<RefCell<TreeNode<T>>>,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}
type TreeNodePtr<T> = Rc<RefCell<TreeNode<T>>>;

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

impl<T: PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T: PartialOrd> PartialOrd for TreeNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: PartialEq + PartialOrd> TreeNode<T> {
    pub fn predecessor(node: TreeNodePtr<T>) -> Option<Rc<RefCell<Self>>> {
        let mut node = node;
        if !node.borrow().left.is_empty() {
            node.borrow().left.max()
        } else {
            let mut parent = node.borrow().parent.upgrade()?;
            while node.as_ref() < parent.as_ref() {
                let next = parent.borrow().parent.upgrade()?;
                node = std::mem::replace(&mut parent, next);
            }
            Some(parent)
        }
    }

    pub fn successor(node: TreeNodePtr<T>) -> Option<TreeNodePtr<T>> {
        let mut node = node;
        if !node.borrow().right.is_empty() {
            node.borrow().right.min()
        } else {
            let mut parent = node.borrow().parent.upgrade()?;
            while node.as_ref() >= parent.as_ref() {
                let next = parent.borrow().parent.upgrade()?;
                node = std::mem::replace(&mut parent, next);
            }
            Some(parent)
        }
    }
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: Option<TreeNodePtr<T>>,
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
}

impl<T: PartialEq + PartialOrd> BinaryTree<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter(self.min())
    }

    pub fn insert(&mut self, x: T) {
        if let Some(cursor) = &mut self.root.clone() {
            loop {
                if x < cursor.borrow().value {
                    if cursor.borrow().left.is_empty() {
                        cursor.borrow_mut().left.root =
                            TreeNode::new_leaf(x, Rc::downgrade(&cursor)).to_root();
                        return;
                    } else if let Some(next) = cursor.clone().borrow().left.root.clone() {
                        *cursor = next;
                    }
                } else if cursor.borrow().right.is_empty() {
                    cursor.borrow_mut().right.root =
                        TreeNode::new_leaf(x, Rc::downgrade(&cursor)).to_root();
                    return;
                } else if let Some(next) = cursor.clone().borrow().right.root.clone() {
                    *cursor = next;
                }
            }
        } else {
            self.root = TreeNode::new(x).to_root();
        }
    }

    fn detach(&mut self, node: TreeNodePtr<T>, new: Option<TreeNodePtr<T>>) {
        if let Some(parent) = node.borrow().parent.upgrade() {
            if let Some(new_node) = &new {
                new_node.borrow_mut().parent = Rc::downgrade(&parent);
            }
            if node.as_ref() < parent.as_ref() {
                parent.borrow_mut().left.root = new;
            } else {
                parent.borrow_mut().right.root = new;
            }
        } else {
            self.root = new;
        }
        node.borrow_mut().parent = Weak::new();
    }

    pub fn remove(&mut self, node: TreeNodePtr<T>) {
        let left_node = node.borrow_mut().left.root.take();
        let right_node = node.borrow_mut().right.root.take();

        if let Some(left_node) = left_node {
            if let Some(right_node) = right_node {
                right_node.borrow_mut().parent = Weak::new();
                let mut right_tree = BinaryTree {
                    root: Some(right_node),
                };
                let right_min = unsafe { right_tree.min().unwrap_unchecked() };
                right_tree.remove(right_min.clone());
                let right_node = right_tree.root.take();

                left_node.borrow_mut().parent = Rc::downgrade(&right_min);
                if let Some(node) = &right_node {
                    node.borrow_mut().parent = Rc::downgrade(&right_min);
                }

                right_min.borrow_mut().left.root = Some(left_node);
                right_min.borrow_mut().right.root = right_node;

                self.detach(node, Some(right_min));
            } else {
                self.detach(node, Some(left_node));
            }
        } else if let Some(right_node) = right_node {
            self.detach(node, Some(right_node))
        } else {
            self.detach(node, None)
        }
    }

    pub fn find(&self, x: &T) -> Option<TreeNodePtr<T>> {
        if let Some(root) = &self.root {
            let mut cursor = root.clone();
            loop {
                if &cursor.borrow().value == x {
                    return Some(cursor);
                } else {
                    cursor = if &cursor.borrow().value < x {
                        cursor.borrow().right.root.clone()
                    } else {
                        cursor.borrow().left.root.clone()
                    }?;
                }
            }
        } else {
            return None;
        }
    }

    pub fn min(&self) -> Option<TreeNodePtr<T>> {
        self.root.as_ref().map(|root| {
            let mut cursor = root.clone();
            while let Some(left) = cursor.clone().borrow().left.root.clone() {
                cursor = left;
            }
            cursor
        })
    }

    pub fn max(&self) -> Option<TreeNodePtr<T>> {
        self.root.as_ref().map(|root| {
            let mut cursor = root.clone();
            while let Some(left) = cursor.clone().borrow().right.root.clone() {
                cursor = left;
            }
            cursor
        })
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

pub struct Iter<T>(Option<TreeNodePtr<T>>);

impl<T: PartialEq + PartialOrd> Iterator for Iter<T> {
    type Item = TreeNodePtr<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = TreeNode::successor(self.0.clone()?);
        std::mem::replace(&mut self.0, next)
    }
}
