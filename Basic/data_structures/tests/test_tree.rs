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
