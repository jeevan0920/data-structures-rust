use std::{cell::RefCell, rc::Rc};

struct BinaryTreeNode {
    value: i32,
    parent: Option<Rc<RefCell<BinaryTreeNode>>>,
    left: Option<Rc<RefCell<BinaryTreeNode>>>,
    right: Option<Rc<RefCell<BinaryTreeNode>>>,
}

impl BinaryTreeNode {
    fn new(value: i32) -> BinaryTreeNode {
        BinaryTreeNode {
            value,
            parent: None,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_binary_tree() {
        let root = BinaryTreeNode::new(1);
        assert_eq!(root.value, 1);
    }

    #[test]
    // create a full complete binary tree 2 levels
    fn create_full_complete_binary_tree() {
        // level 1
        let root = Rc::new(RefCell::new(BinaryTreeNode::new(1)));

        // level 2
        let left = Rc::new(RefCell::new(BinaryTreeNode::new(2)));
        let right = Rc::new(RefCell::new(BinaryTreeNode::new(3)));
        root.borrow_mut().left = Some(left.clone());
        root.borrow_mut().right = Some(right.clone());
        left.borrow_mut().parent = Some(root.clone());
        right.borrow_mut().parent = Some(root.clone());

        // level 1 nodes
        println!("root: {}", root.borrow().value);

        // level 2 nodes
        println!(
            "left: {}",
            root.borrow().left.as_ref().unwrap().borrow().value
        );
        println!(
            "right: {}",
            root.borrow().right.as_ref().unwrap().borrow().value
        );
    }
}
