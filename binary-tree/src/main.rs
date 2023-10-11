use std::fmt::Debug;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct BinaryTree<T> {
    pub value: Option<T>,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}

impl<T: PartialEq + PartialOrd + Clone + Debug> BinaryTree<T> {
    pub fn new(value: T) -> Self {
        BinaryTree {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn insert(&mut self, value: T) -> &Self {
        match &self.value {
            Some(current) => loop {
                if value < *current {
                    match &self.left {
                        Some(left_node_value) => {
                            let mut left_node = left_node_value.clone();
                            let left = left_node.insert(value.clone()).clone();
                            self.left = Some(Box::new(left));
                            return self;
                        }
                        None => {
                            self.left = Some(Box::new(Self::new(value.clone())));
                            return self;
                        }
                    }
                } else {
                    match &self.right {
                        Some(right_node_value) => {
                            let mut right_node = right_node_value.clone();
                            let right = right_node.insert(value.clone()).clone();
                            self.right = Some(Box::new(right));
                        }
                        None => {
                            self.right = Some(Box::new(Self::new(value.clone())));
                            return self;
                        }
                    }
                }

                return self;
            },
            None => {
                self.value = Some(value.clone());
                return self;
            }
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new(4);
    tree.insert(5);
    tree.insert(3);
    tree.insert(2);
    tree.insert(17);

    dbg!(tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_tree() {
        let tree = BinaryTree::new(1);
        if let Some(root) = tree.value {
            assert_eq!(root, 1);
        }
        assert_eq!(tree.right, None);
        assert_eq!(tree.left, None);
    }

    #[test]
    fn insert_left() {
        let mut tree = BinaryTree::new(2);
        tree.insert(1);

        if let Some(root) = tree.value {
            assert_eq!(root, 2);
        }

        if let Some(left_node) = tree.left {
            if let Some(root) = left_node.value {
                assert_eq!(root, 1);
            }
        }

        assert_eq!(tree.right, None);
    }

    #[test]
    fn insert_right() {
        let mut tree = BinaryTree::new(2);
        tree.insert(4);

        if let Some(root) = tree.value {
            assert_eq!(root, 2);
        }
        if let Some(right_node) = tree.right {
            if let Some(root) = right_node.value {
                assert_eq!(root, 4);
            }
        }

        assert_eq!(tree.left, None);
    }
}
