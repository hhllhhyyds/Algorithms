use std::fmt::Display;

#[derive(Debug)]
pub struct SingleLinkedNode<T> {
    value: T,
    next: SingleLinkedList<T>,
}

impl<T> SingleLinkedNode<T> {
    pub fn new(x: T) -> Self {
        Self {
            value: x,
            next: SingleLinkedList::default(),
        }
    }
}

#[derive(Debug)]
pub struct SingleLinkedList<T> {
    head: Option<Box<SingleLinkedNode<T>>>,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self)
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn split(node: &mut SingleLinkedNode<T>) -> SingleLinkedList<T> {
        std::mem::take(&mut node.next)
    }

    pub fn concat(node: &mut SingleLinkedNode<T>, list: SingleLinkedList<T>) {
        assert!(std::mem::replace(&mut node.next, list).is_empty())
    }

    pub fn push_head(&mut self, x: T) {
        let mut node = Box::new(SingleLinkedNode::new(x));
        Self::concat(&mut node, std::mem::take(self));
        self.head = Some(node)
    }

    pub fn pop_head(&mut self) -> Option<T> {
        let head = self.head.take();
        head.map(|node| {
            let (value, list) = (node.value, node.next);
            *self = list;
            value
        })
    }

    pub fn push_tail(&mut self, x: T) {
        let tail = SingleLinkedList {
            head: Some(Box::new(SingleLinkedNode::new(x))),
        };

        let mut list = self;
        loop {
            if list.is_empty() {
                *list = tail;
                break;
            } else {
                unsafe {
                    list = &mut list.head.as_mut().unwrap_unchecked().next;
                }
            }
        }
    }

    pub fn insert_after(node: &mut SingleLinkedNode<T>, x: T) {
        let mut list = Self::split(node);
        list.push_head(x);
        Self::concat(node, list);
    }

    pub fn pop_after(node: &mut SingleLinkedNode<T>) -> Option<T> {
        let mut list = Self::split(node);
        let value = list.pop_head();
        Self::concat(node, list);
        value
    }
}

impl<T> Default for SingleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> SingleLinkedList<T> {
    pub fn find(&self, x: &T) -> Option<&SingleLinkedNode<T>> {
        let mut head = &self.head;
        while let Some(node) = head {
            if node.value.eq(x) {
                return Some(node);
            } else {
                head = &node.next.head;
            }
        }
        None
    }

    pub fn find_mut(&mut self, x: &T) -> Option<&mut SingleLinkedNode<T>> {
        let mut head = &mut self.head;
        while let Some(node) = head {
            if node.value.eq(x) {
                return Some(node);
            } else {
                head = &mut node.next.head;
            }
        }
        None
    }

    pub fn pop_match(&mut self, x: &T) -> Option<T> {
        let mut list = self;

        while list.head.is_some() {
            unsafe {
                if list.head.as_ref().unwrap_unchecked().value.ne(x) {
                    list = &mut list.head.as_mut().unwrap_unchecked().next;
                } else {
                    let node = list.head.take().unwrap_unchecked();
                    let (x, tail) = (node.value, node.next);
                    *list = tail;
                    return Some(x);
                }
            }
        }

        None
    }
}

impl<T: Display> Display for SingleLinkedNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: Display> Display for SingleLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SingleLinkedList [")?;
        for node in self.iter() {
            node.fmt(f)?;
            if node.next.head.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;

        Ok(())
    }
}

pub struct Iter<'a, T>(&'a SingleLinkedList<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a SingleLinkedNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.as_ref().map(|node| {
            self.0 = &node.next;
            node.as_ref()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut list = SingleLinkedList::new();

        for i in 0..10 {
            list.push_head(i);
        }

        println!("List is {list}");

        for _ in 0..4 {
            list.pop_head();
        }

        println!("List is {list}");

        assert!(list.find(&3).is_some());
        assert!(list.find(&3).unwrap().value == 3);

        assert!(list.find(&6).is_none());

        assert!(list.find_mut(&3).is_some());
        assert!({
            list.find_mut(&3).unwrap().value = 100;
            list.find(&100).is_some() && list.find(&3).is_none()
        });

        println!("List is {list}");

        assert!(list.find(&2).is_some());
        let x = list.pop_match(&2);
        assert!(list.find(&2).is_none());
        assert!(x.unwrap() == 2);

        println!("List is {list}");

        for i in 6..10 {
            list.push_tail(i);
        }

        println!("List is {list}");
    }
}
