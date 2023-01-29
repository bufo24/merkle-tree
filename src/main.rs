use std::{env, fs};

use merkle_tree::structs::merkle_tree::MerkleTree;

fn main() {
    // let mut leafs = Vec::<Leaf>::new();
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // let elements: Vec<&str> = contents.split(' ').collect();

    let mut tree = MerkleTree::new("ROOT".to_string());

    tree = tree.set_left_child("left".to_string());
    tree = tree.set_right_child("right".to_string());

    println!("{}", tree.get_hash());
    // let root = tree.get_root();
    // println!("{}", root.get_hash());
    // tree.print(None);
}
