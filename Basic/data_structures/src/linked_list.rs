// use std::ops::DerefMut;

use std::fmt::Display;

#[derive(Debug)]
pub struct SingleLinkedNode<T> {
    value: T,
    next: SingleLinkedList<T>,
}

#[derive(Debug)]
pub struct SingleLinkedList<T> {
    node: Option<Box<SingleLinkedNode<T>>>,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self { node: None }
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter(self)
    }

    // fn iter_mut(&mut self) -> IterMut<'_, T> {
    //     IterMut(self)
    // }
}

impl<T> Default for SingleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> SingleLinkedList<T> {
    pub fn find(&self, x: &T) -> Option<&SingleLinkedNode<T>> {
        match &self.node {
            Some(node) => {
                if node.value.eq(x) {
                    Some(node)
                } else {
                    node.next.find(x)
                }
            }
            None => None,
        }
    }

    pub fn find_mut(&mut self, x: &T) -> Option<&mut SingleLinkedNode<T>> {
        match &mut self.node {
            Some(node) => {
                if node.value.eq(x) {
                    Some(node)
                } else {
                    node.next.find_mut(x)
                }
            }
            None => None,
        }
    }

    pub fn push_head(&mut self, x: T) {
        let head_node = Box::new(SingleLinkedNode {
            value: x,
            next: SingleLinkedList {
                node: self.node.take(),
            },
        });

        self.node = Some(head_node);
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.node.as_ref()?;

        let node = self.node.take().unwrap();
        let (list, x) = (node.next, node.value);
        *self = list;
        Some(x)
    }

    pub fn delete(&mut self, x: &T) {
        if self.node.is_none() {
            return;
        }

        let mut list = self;

        while list.node.is_some() {
            if list.node.as_ref().unwrap().value.ne(x) {
                list = &mut list.node.as_mut().unwrap().next;
            } else {
                let node = list.node.take().unwrap();
                let (_, left_list) = (node.value, node.next);
                *list = left_list;
                break;
            }
        }
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
            write!(f, ", ")?;
        }
        write!(f, "]")?;

        Ok(())
    }
}

struct Iter<'a, T>(&'a SingleLinkedList<T>);

// struct IterMut<'a, T>(&'a mut SingleLinkedList<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a SingleLinkedNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.node.as_ref().map(|node| {
            let ret = node.as_ref();
            self.0 = &node.next;
            ret
        })
    }
}

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut SingleLinkedNode<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0 .0.as_mut().take().map(move |node| node.as_mut())
//     }
// }

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
        list.delete(&2);
        assert!(list.find(&2).is_none());

        println!("List is {list}");
    }
}
