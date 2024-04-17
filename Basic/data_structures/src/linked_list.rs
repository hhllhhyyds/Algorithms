#[derive(Debug)]
pub struct SingleLinkedNode<T> {
    value: T,
    next: SingleLinkedList<T>,
}

#[derive(Debug)]
pub struct SingleLinkedList<T>(Option<Box<SingleLinkedNode<T>>>);

impl<T: PartialEq> SingleLinkedList<T> {
    pub fn search(&self, x: &T) -> Option<&SingleLinkedNode<T>> {
        match &self.0 {
            Some(node) => {
                if node.value.eq(x) {
                    return Some(node);
                } else {
                    node.next.search(x)
                }
            }
            None => None,
        }
    }
}
