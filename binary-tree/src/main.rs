use std::fmt::Debug;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct BTree<T> {
    pub value: Option<T>,
    pub left: Option<Box<BTree<T>>>,
    pub right: Option<Box<BTree<T>>>,
}

impl<T: PartialEq + PartialOrd + Clone + Debug> BTree<T> {
    pub fn new(value: T) -> Self {
        BTree {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: BTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: BTree<T>) -> Self {
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
    let mut tree = BTree::new(4);
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
        let tree = BTree::new(1);

        assert_eq!(tree.value, 1);
    }

    #[test]
    fn insert_left() {
        let tree = BTree::new(1).left(BTree::new(2));

        if let Some(node) = tree.left {
            assert_eq!(node.value, 2);
        }

        assert_eq!(tree.right, None);
    }

    #[test]
    fn insert_right() {
        let tree = BTree::new(1).right(BTree::new(2));

        if let Some(node) = tree.right {
            assert_eq!(node.value, 2);
        }

        assert_eq!(tree.left, None);
    }
}
