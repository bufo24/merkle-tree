#[cfg(test)]
mod tests {
    use merkle_tree::structs::merkle_tree::MerkleTree;

    #[test]
    fn create_new_tree() {
        let tree = MerkleTree::new("root".to_string());

        assert_eq!(tree.get_value(), "root");
    }

    #[test]
    fn insert_left() {
        let tree =
            MerkleTree::new("root".to_string()).set_left_child(MerkleTree::new("left".to_string()));

        if let Some(node) = tree.left_child {
            assert_eq!(node.get_value(), "left");
        }

        assert!(tree.right_child.is_none());
    }

    // #[test]
    // fn insert_right() {
    //     let tree = BinaryTree::new(1).right(BinaryTree::new(2));

    //     if let Some(node) = tree.right {
    //         assert_eq!(node.value, 2);
    //     }

    //     assert_eq!(tree.left, None);
    // }
}
