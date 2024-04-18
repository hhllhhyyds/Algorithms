use crate::linked_list::SingleLinkedList;

pub trait Stack<T> {
    fn push(&mut self, x: T);
    fn pop(&mut self) -> Option<T>;
    fn size(&self) -> usize;
    fn new() -> Self;
}

impl<T> Stack<T> for Vec<T> {
    fn push(&mut self, x: T) {
        self.push(x)
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn new() -> Self {
        Vec::new()
    }
}

impl<T> Stack<T> for SingleLinkedList<T> {
    fn push(&mut self, x: T) {
        self.push_head(x)
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_head()
    }

    fn size(&self) -> usize {
        self.iter().count()
    }

    fn new() -> Self {
        SingleLinkedList::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_stack() {
        let mut stack: Vec<_> = Stack::new();
        for i in 0..10 {
            stack.push(i);
        }

        assert!(stack.size() == 10);

        for i in (0..10).rev() {
            assert!(i == Stack::pop(&mut stack).unwrap());
        }

        assert!(stack.size() == 0);
    }

    #[test]
    fn test_list_stack() {
        let mut stack: SingleLinkedList<_> = Stack::new();
        for i in 0..10 {
            stack.push(i);
        }

        assert!(stack.size() == 10);

        for i in (0..10).rev() {
            assert!(i == Stack::pop(&mut stack).unwrap());
        }

        assert!(stack.size() == 0);
    }
}
