use data_structures::linked_list::*;

fn list_content_eq_to<'a, T: PartialEq + 'a>(
    list: &SingleLinkedList<T>,
    iter: impl Iterator<Item = &'a T>,
) -> bool {
    for (a, b) in list.iter().zip(iter) {
        if a.value != *b {
            return false;
        }
    }

    true
}

#[test]
fn test_create_list() {
    let mut list = SingleLinkedList::new();
    assert!(list.len() == 0);

    for i in 0..100 {
        list.push_head(i);
    }

    let iter: Vec<_> = (0..100).into_iter().collect();
    assert!(list_content_eq_to(&list, iter.iter().rev()));

    list = SingleLinkedList::default();
    assert!(list.len() == 0 && list.is_empty());
    for i in 0..100 {
        list.push_tail(i);
    }

    assert!(!list.is_empty() && list.len() == 100);

    assert!(list_content_eq_to(&list, iter.iter()))
}

#[test]
fn test_split_concat() {
    let mut list = SingleLinkedList::new();

    for i in (0..100).rev() {
        list.push_head(i);
    }

    let tail = {
        let middle = list.find_mut(&49);
        SingleLinkedList::split(middle.unwrap())
    };

    assert!(list.len() == 50);
    assert!(tail.len() == 50);

    let iter1: Vec<_> = (0..50).into_iter().collect();
    assert!(list_content_eq_to(&list, iter1.iter()));

    let iter2: Vec<_> = (50..100).into_iter().collect();
    assert!(list_content_eq_to(&tail, iter2.iter()));

    {
        let middle = list.find_mut(&49);
        SingleLinkedList::concat(middle.unwrap(), tail);
        let iter3: Vec<_> = (0..100).into_iter().collect();
        assert!(list_content_eq_to(&list, iter3.iter()));
    }
}

#[test]
fn test_push_pop() {
    let mut list = SingleLinkedList::new();

    for i in (0..100).rev() {
        list.push_head(i);
    }

    list.push_head(-1);
    assert!(list.len() == 101);
    let iter: Vec<_> = (-1..100).into_iter().collect();
    assert!(list_content_eq_to(&list, iter.iter()));

    list.push_tail(100);
    assert!(list.len() == 102);
    let iter: Vec<_> = (-1..101).into_iter().collect();
    assert!(list_content_eq_to(&list, iter.iter()));

    list.pop_head();
    assert!(list.len() == 101);
    let iter: Vec<_> = (0..101).into_iter().collect();
    assert!(list_content_eq_to(&list, iter.iter()));
}

#[test]
fn test_insert_pop() {
    let mut list = SingleLinkedList::new();

    for i in (0..100).rev() {
        list.push_head(i);
    }

    let middle = list.find_mut(&49);
    SingleLinkedList::insert_after(middle.unwrap(), -99);
    assert!(list.len() == 101);
    let mut iter: Vec<_> = (0..100).into_iter().collect();
    iter.insert(50, -99);
    assert!(list_content_eq_to(&list, iter.iter()));

    let middle = list.find_mut(&49);
    SingleLinkedList::pop_after(middle.unwrap());
    let iter: Vec<_> = (0..100).into_iter().collect();
    assert!(list_content_eq_to(&list, iter.iter()));
}

#[test]
fn test_find_match() {
    let mut list = SingleLinkedList::new();

    for i in (0..100).rev() {
        list.push_head(i);
    }

    assert!(list.find(&37).unwrap().value == 37);
    list.find_mut(&37).unwrap().value = -37;
    assert!(list.find(&37).is_none());
    assert!(list.find(&-37).is_some());

    list.pop_match(&-37);
    assert!(list.len() == 99);
    assert!(list.find(&-37).is_none());
}
