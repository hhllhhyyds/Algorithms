use std::fmt::Display;

#[derive(Debug)]
pub struct SingleLinkedNode<T> {
    value: T,
    next: SingleLinkedList<T>,
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

    pub fn push_head(&mut self, x: T) {
        let head = Box::new(SingleLinkedNode {
            value: x,
            next: SingleLinkedList {
                head: self.head.take(),
            },
        });

        self.head = Some(head);
    }

    pub fn pop_head(&mut self) -> Option<T> {
        let head = self.head.take()?;
        let (list, x) = (head.next, head.value);
        *self = list;
        Some(x)
    }

    pub fn push_tail(&mut self, x: T) {
        let tail = Some(Box::new(SingleLinkedNode {
            value: x,
            next: SingleLinkedList { head: None },
        }));

        let mut last = &mut self.head;

        loop {
            if last.is_none() {
                *last = tail;
                break;
            } else {
                unsafe {
                    last = &mut last.as_mut().unwrap_unchecked().next.head;
                }
            }
        }
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
        if self.head.is_some() {
            let mut list = self;

            while list.head.is_some() {
                unsafe {
                    if list.head.as_ref().unwrap_unchecked().value.ne(x) {
                        list = &mut list.head.as_mut().unwrap_unchecked().next;
                    } else {
                        let node = list.head.take().unwrap_unchecked();
                        let (x, left_list) = (node.value, node.next);
                        *list = left_list;
                        return Some(x);
                    }
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
