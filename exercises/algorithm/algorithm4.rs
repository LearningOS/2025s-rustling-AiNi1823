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

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            let new_node = Box::new(TreeNode::<T>::new(value));
            self.root = Some(new_node);
            return;
        }

        let mut parent = &mut self.root;

        while (value < parent.as_ref().unwrap().value && parent.as_ref().unwrap().left.is_some())
            || (value > parent.as_ref().unwrap().value && parent.as_ref().unwrap().right.is_some())
        {
            if value < parent.as_ref().unwrap().value {
                parent = &mut parent.as_mut().unwrap().left;
            } else {
                parent = &mut parent.as_mut().unwrap().right;
            }
        }

        if value != parent.as_ref().unwrap().value {
            parent.as_mut().unwrap().insert(value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut next = &self.root;

        while next.is_some() {
            let next_box = next.as_ref().unwrap();

            if next_box.value == value {
                return true;
            } else if value < next_box.value {
                next = &next_box.left;
            } else {
                next = &next_box.right;
            }
        }

        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if value < self.value {
            let new_node = Box::new(TreeNode::<T>::new(value));
            self.left = Some(new_node);
        } else if value > self.value {
            let new_node = Box::new(TreeNode::<T>::new(value));
            self.right = Some(new_node);
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


