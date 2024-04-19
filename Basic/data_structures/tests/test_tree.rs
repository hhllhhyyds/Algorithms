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
