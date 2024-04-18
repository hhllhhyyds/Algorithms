use crate::linked_list::SingleLinkedList;

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

#[cfg(test)]
mod tests {
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
}
