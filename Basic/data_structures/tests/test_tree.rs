use data_structures::tree::BinaryTree;

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
        let mut input = (0..200).map(|_| rng.gen_range(0..1000)).collect::<Vec<_>>();

        let mut tree = BinaryTree::new();

        for x in &input {
            tree.insert(*x);
        }

        input.sort();
        for (a, b) in input.iter().zip(tree.iter()) {
            assert!(*a == b.as_ref().borrow().value);
        }
    }
}

fn verify_tree_order<T: PartialEq + PartialOrd>(tree: &BinaryTree<T>) {
    let mut root_count = 0;
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
    assert!(root_count == 1);
}

#[test]
fn test_weak_to_parent() {
    let mut rng = rand::thread_rng();

    let input = (0..200).map(|_| rng.gen_range(0..1000)).collect::<Vec<_>>();

    let mut tree = BinaryTree::new();

    for x in &input {
        tree.insert(*x);
    }

    verify_tree_order(&tree);
}
