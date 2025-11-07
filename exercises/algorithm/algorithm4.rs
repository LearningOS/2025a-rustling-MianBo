/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
        } else {
            self.root.as_mut().unwrap().insert(value);
        }
    }

    fn search(&self, value: T) -> bool {
        let mut current_node = self.root.as_ref();
        
        while let Some(node) = current_node {
            match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => current_node = node.left.as_ref(),
                Ordering::Greater => current_node = node.right.as_ref(),
            }
        }
        
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {
                return;
            },
            Ordering::Less => {
                if let Some(ref mut left_node) = self.left {
                    left_node.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            },
            Ordering::Greater => {
                if let Some(ref mut right_node) = self.right {
                    right_node.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
                assert_eq!(node.value, 1);
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}