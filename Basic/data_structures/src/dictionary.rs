pub trait Dictionary<K: PartialEq + PartialOrd, V> {
    fn find(&self, key: &K) -> Option<&V>;
    fn find_mut(&mut self, key: &mut K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V);
    fn remove(&mut self, value: &mut V);
    fn max(&self) -> Option<&V>;
    fn min(&self) -> Option<&V>;
    fn max_mut(&mut self) -> Option<&mut V>;
    fn min_mut(&mut self) -> Option<&mut V>;
    fn predecessor(&self, key: &K) -> Option<&V>;
    fn successor(&self, key: &K) -> Option<&V>;
    fn predecessor_mut(&mut self, key: &K) -> Option<&mut V>;
    fn successor_mut(&mut self, key: &K) -> Option<&mut V>;
}

pub struct Sorted<T>(pub T);
pub struct Unsorted<T>(pub T);

struct DictionaryItem<K, V> {
    key: K,
    value: V,
    index: usize,
}

impl<K: PartialEq + PartialOrd, V> Dictionary<K, V> for Sorted<Vec<DictionaryItem<K, V>>> {
    fn find(&self, key: &K) -> Option<&V> {
        todo!()
    }

    fn find_mut(&mut self, key: &mut K) -> Option<&mut V> {
        todo!()
    }

    fn insert(&mut self, key: K, value: V) {
        todo!()
    }

    fn remove(&mut self, value: &mut V) {
        todo!()
    }

    fn max(&self) -> Option<&V> {
        self.0.last().map(|item| &item.value)
    }

    fn min(&self) -> Option<&V> {
        self.0.first().map(|item| &item.value)
    }

    fn max_mut(&mut self) -> Option<&mut V> {
        self.0.last_mut().map(|item| &mut item.value)
    }

    fn min_mut(&mut self) -> Option<&mut V> {
        self.0.first_mut().map(|item| &mut item.value)
    }

    fn predecessor(&self, key: &K) -> Option<&V> {}

    fn successor(&self, key: &K) -> Option<&V> {
        todo!()
    }

    fn predecessor_mut(&mut self, key: &K) -> Option<&mut V> {
        todo!()
    }

    fn successor_mut(&mut self, key: &K) -> Option<&mut V> {
        todo!()
    }
}
