use sha256::digest;

#[derive(Clone)]
pub struct MerkleTree {
    data: String,
    pub left_child: Option<Box<MerkleTree>>,
    pub right_child: Option<Box<MerkleTree>>,
}

impl MerkleTree {
    pub fn new(data: String) -> MerkleTree {
        MerkleTree {
            data,
            left_child: None,
            right_child: None,
        }
    }

    pub fn get_value(&self) -> &str {
        &self.data
    }

    pub fn get_hash(&self) -> String {
        let mut data = self.get_value().to_string();
        if self.left_child.is_some() {
            let left = self.left_child.as_ref().unwrap();
            data.push_str(&left.get_hash());
        }
        if self.right_child.is_some() {
            let right = self.right_child.as_ref().unwrap();
            data.push_str(&right.get_hash());
        }
        digest(data.to_string())
    }

    pub fn set_left_child(mut self, data: String) -> Self {
        let merkle_tree = Self::new(data);
        self.left_child = Some(Box::new(merkle_tree));
        self
    }

    // pub fn get_left_child(&mut self) -> Option<Self> {
    // self.left_child
    // Some(self.left_child.unwrap().as_ref())
    // unsafe { self.left_child.as_mut().map(|leaf| *leaf.as_ref()) }
    // unsafe { Some(self.left_child.as_mut().unwrap().to_owned().as_ref()) }
    // unsafe { self.left_child.as_mut().map(|leaf| leaf.as_ref()) }
    // unsafe { self.left_child.as_mut().map(|leaf| leaf.as_mut()) }
    // }

    pub fn has_left_child(&self) -> bool {
        self.left_child.is_some()
    }

    pub fn set_right_child(mut self, data: String) -> Self {
        let merkle_tree = Self::new(data);
        self.right_child = Some(Box::new(merkle_tree));
        self
    }

    // pub fn get_right_child(&mut self) -> Option<&Self> {
    // Some(self.right_child.unwrap().as_ref())
    // unsafe { Some(self.right_child.as_mut().unwrap().to_owned().as_ref()) }
    // }

    pub fn has_right_child(&self) -> bool {
        self.right_child.is_some()
    }

    //     pub fn add_child(&mut self, value: &str) -> bool {
    //         let mut leaf = MerkleTree::new(value.to_string(), None, None);
    //         if !self.has_left_child() {
    //             let box_leaf = Box::new(leaf);
    //             self.set_left_child(box_leaf);
    //             println!("added '{}' on left child of {}", value, self.data);
    //             return true;
    //         } else if !self.has_right_child() {
    //             let box_leaf = Box::new(leaf);
    //             self.set_right_child(box_leaf);
    //             println!("added '{}' on right child of {}", value, self.data);
    //             return true;
    //         }
    //         false
    //     }
}
