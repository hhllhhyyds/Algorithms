use std::collections::HashSet;

use data_structures::tree::{BinaryTree, TreeNode};

use rand::Rng;

#[test]
fn test_display() {
    let mut rng = rand::thread_rng();

    let mut tree = BinaryTree::new();
    for _ in 0..15 {
        tree.insert(rng.gen_range(0..100));
    }

    println!("Tree = \n{tree}");
}

#[test]
fn test_find() {
    let mut rng = rand::thread_rng();
    let input = (0..20).map(|_| rng.gen_range(0..100)).collect::<Vec<_>>();

    let mut tree = BinaryTree::new();

    for x in &input {
        tree.insert(*x);
    }

    for x in &input {
        assert!(tree.find(&x).is_some())
    }

    for _ in 0..20 {
        let x = rng.gen_range(100..200);
        assert!(tree.find(&x).is_none())
    }
}

#[test]
fn test_min_max() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let mut input = (0..20).map(|_| rng.gen_range(0..100)).collect::<Vec<_>>();

        let mut tree = BinaryTree::new();

        for x in &input {
            tree.insert(*x);
        }

        input.sort();

        assert!(input.first().unwrap() == &tree.min().unwrap().as_ref().borrow().value);
        assert!(input.last().unwrap() == &tree.max().unwrap().as_ref().borrow().value);
    }
}

#[test]
fn test_iter() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let mut input = (0..5).map(|_| rng.gen_range(0..1000)).collect::<Vec<_>>();

        let mut tree = BinaryTree::new();

        for x in &input {
            tree.insert(*x);
        }

        println!("tree = {tree}");
        for x in tree.iter() {
            println!("node = {}", x.borrow().value);
        }

        input.sort();
        assert!(input.iter().count() == tree.iter().count());
        for (a, b) in input.iter().zip(tree.iter()) {
            assert!(*a == b.as_ref().borrow().value);
        }
    }
}

fn verify_tree_order<T: PartialEq + PartialOrd>(tree: &BinaryTree<T>) {
    let mut root_count = 0;
    if !tree.is_empty() {
        for item in tree.iter() {
            let parent = item.borrow().parent.upgrade();
            if parent.is_none() {
                root_count += 1;
            } else {
                let parent = parent.unwrap();
                if parent.borrow().value > item.borrow().value {
                    assert!(std::ptr::addr_eq(
                        parent.borrow().left.root.as_ref().unwrap().as_ptr(),
                        item.as_ptr()
                    ))
                } else {
                    assert!(std::ptr::addr_eq(
                        parent.borrow().right.root.as_ref().unwrap().as_ptr(),
                        item.as_ptr()
                    ))
                }
            }
        }
        assert!(
            root_count == if tree.is_empty() { 0 } else { 1 },
            "root count = {root_count}"
        );
    }
}

#[test]
fn test_weak_to_parent() {
    let mut rng = rand::thread_rng();
    let size = 16;
    let input = (0..size)
        .map(|_| rng.gen_range(0..1000))
        .collect::<Vec<_>>();

    let mut tree = BinaryTree::new();

    for x in &input {
        tree.insert(*x);
    }

    assert!(
        tree.iter().count() == size,
        "iter count = {}, size = {size}",
        tree.iter().count()
    );
    println!("tree = {tree}");

    verify_tree_order(&tree);
}

#[test]
fn test_remove() {
    let mut rng = rand::thread_rng();

    let input = (0..7000)
        .map(|_| rng.gen_range(0..1000))
        .collect::<HashSet<_>>();
    let mut len = input.len();
    println!("set length = {}, set = {:?}", len, input);

    let mut tree = BinaryTree::new();

    for x in &input {
        tree.insert(*x);
    }

    for x in input {
        {
            println!("x to be removed = {x}");
            let node = tree.find(&x).unwrap();
            tree.remove(node);
            len -= 1;
        }
        assert!(
            tree.iter().count() == len,
            "{} != {}",
            tree.iter().count(),
            len
        );
        assert!(tree.find(&x).is_none());
        verify_tree_order(&tree);
    }
}

#[test]
fn test_predecessor_successor() {
    let mut rng = rand::thread_rng();
    let size = 7000;

    let input = (0..size)
        .map(|_| rng.gen_range(0..10000))
        .collect::<HashSet<_>>();

    let mut tree = BinaryTree::new();

    let mut sorted = Vec::new();
    for x in &input {
        tree.insert(*x);
        sorted.push(*x);
    }
    sorted.sort();

    for i in 1..(sorted.len() - 1) {
        let pre = TreeNode::predecessor(tree.find(&sorted[i]).unwrap());
        let suc = TreeNode::successor(tree.find(&sorted[i]).unwrap());
        if pre.is_some() {
            assert!(pre.unwrap().borrow().value == sorted[i - 1]);
        }
        if suc.is_some() {
            assert!(suc.unwrap().borrow().value == sorted[i + 1]);
        }
    }
}
