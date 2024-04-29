use crate::{linked_list::SingleLinkedList, tree::BinaryTree};

pub trait Queue<T> {
    fn enqueue(&mut self, x: T);
    fn dequeue(&mut self) -> Option<T>;
    fn size(&self) -> usize;
    fn new() -> Self;
}

impl<T> Queue<T> for Vec<T> {
    fn enqueue(&mut self, x: T) {
        self.push(x)
    }

    fn dequeue(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(self.remove(0))
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn new() -> Self {
        Vec::new()
    }
}

impl<T> Queue<T> for SingleLinkedList<T> {
    fn enqueue(&mut self, x: T) {
        self.push_tail(x)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.pop_head()
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn new() -> Self {
        SingleLinkedList::new()
    }
}

#[derive(Debug)]
struct Node<T>(usize, T);
impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: std::fmt::Debug> Queue<T> for BinaryTree<Node<T>> {
    fn enqueue(&mut self, x: T) {
        let index = self
            .max()
            .map(|node| node.borrow().value.0 + 1)
            .unwrap_or(0);
        self.insert(Node(index, x))
    }

    fn dequeue(&mut self) -> Option<T> {
        let min = self.min()?;
        self.remove(min.clone());
        Some(std::rc::Rc::try_unwrap(min).unwrap().into_inner().value.1)
    }

    fn size(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        let max_index = self.max().unwrap().borrow().value.0;
        let min_index = self.min().unwrap().borrow().value.0;
        max_index - min_index + 1
    }

    fn new() -> Self {
        BinaryTree::new()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_vec_queue() {
        let mut queue: Vec<_> = Queue::new();
        for i in 0..10 {
            queue.enqueue(i);
        }

        assert!(queue.size() == 10);

        for i in 0..10 {
            assert!(i == queue.dequeue().unwrap());
        }

        assert!(queue.size() == 0);
    }

    #[test]
    fn test_list_queue() {
        let mut queue: SingleLinkedList<_> = Queue::new();
        for i in 0..10 {
            queue.enqueue(i);
        }

        assert!(queue.size() == 10);

        for i in 0..10 {
            assert!(i == queue.dequeue().unwrap());
        }

        assert!(queue.size() == 0);
    }

    #[test]
    fn test_tree_queue() {
        let mut queue: BinaryTree<_> = Queue::new();
        let mut rng = rand::thread_rng();
        let input = (0..100)
            .map(|_| rng.gen_range(0..10000))
            .collect::<Vec<_>>();
        for x in &input {
            queue.enqueue(*x);
        }

        assert!(queue.size() == 100);

        for i in 0..100 {
            assert!(input[i] == queue.dequeue().unwrap());
        }

        assert!(queue.size() == 0);
    }
}
