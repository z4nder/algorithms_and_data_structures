#[derive(PartialEq, PartialOrd, Debug)]
pub struct BTree<T> {
    pub value: T,
    pub left: Option<Box<BTree<T>>>,
    pub right: Option<Box<BTree<T>>>,
}

impl<T> BTree<T> {
    pub fn new(value: T) -> Self {
        BTree {
            value,
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
}

fn main() {
    let tree = BTree::new(1).left(BTree::new(2)).right(BTree::new(3));

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
