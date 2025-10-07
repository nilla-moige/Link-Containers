#[allow(unused_imports)]
use std::{cmp::Ord, mem};

#[derive(Clone, Debug)]
pub enum TreeNode<T: Ord> {
    Leaf,
    Node(T, Box<TreeNode<T>>, Box<TreeNode<T>>),
}

// Provided functions
impl<T: Ord> TreeNode<T> {
    pub fn height(&self) -> usize {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => 1 + std::cmp::max(left.height(), right.height()),
        }
    }

    /// Verifies that the tree is a binary search tree
    fn is_bst(&self) -> bool {
        fn is_bst_helper<T: Ord>(tree: &TreeNode<T>, min: Option<&T>, max: Option<&T>) -> bool {
            match tree {
                TreeNode::Leaf => true,
                TreeNode::Node(value, left, right) => {
                    match min {
                        Some(min) => {
                            if value <= min {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    match max {
                        Some(max) => {
                            if value >= max {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    is_bst_helper(left, min, Some(value)) && is_bst_helper(right, Some(value), max)
                }
            }
        }
        is_bst_helper(self, None, None)
    }

    /// Verifies that the tree is balanced
    pub fn is_balanced(&self) -> bool {
        match self {
            TreeNode::Leaf => true,
            TreeNode::Node(_, left, right) => {
                let left_height = left.height();
                let right_height = right.height();
                let diff = (left_height as i32 - right_height as i32).abs();
                diff <= 1 && left.is_balanced() && right.is_balanced()
            }
        }
    }

    /// Verifies that the tree is a valid balanced binary search tree
    pub fn validate(&self) -> bool {
        self.is_bst() && self.is_balanced()
    }
}

// Required functions
impl<T: Ord> TreeNode<T> {
    /// Creates a new `TreeNode<T>` with value `value` and children `left` and `right`
    pub fn node(value: T, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
        TreeNode::Node(value, Box::new(left), Box::new(right))
    }

    /// Creates a new `TreeNode<T>` with no children
    pub fn new() -> TreeNode<T> {
        TreeNode::Leaf
    }

    /// Inserts a new node with value `value` into the tree. If the value already exists in the tree,
    /// the function does nothing.
    ///
    /// After insertion, the tree is rebalanced if necessary
    pub fn insert(&mut self, value: T) {
        match self {
            TreeNode::Leaf => {
                *self = TreeNode::Node(value, Box::new(TreeNode::Leaf), Box::new(TreeNode::Leaf));
            }
            TreeNode::Node(val, left, right) => {
                if value < *val {
                    left.insert(value);
                } else if value > *val {
                    right.insert(value);
                }
            }
        }
        self.rebalance();
    }

    /// Computes the balance factor of the tree (the difference between the height of the left and right subtrees)
    fn balance_factor(&self) -> i32 {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => {
                fn height<U: Ord>(node: &TreeNode<U>) -> i32 {
                    match node {
                        TreeNode::Leaf => 0,
                        TreeNode::Node(_, l, r) => 1 + std::cmp::max(height(l), height(r)),
                    }
                }
                height(left) - height(right)
            }
        }
    }

    /// Performs a left rotation on the tree
    pub fn left_rotate(&mut self) {
        let old_root = mem::take(self);
        match old_root {
            TreeNode::Node(value, left, right) => match *right {
                TreeNode::Leaf => {
                    *self = TreeNode::Node(value, left, Box::new(TreeNode::Leaf));
                }
                TreeNode::Node(value2, left2, right2) => {
                    let new_left = TreeNode::Node(value, left, left2);
                    *self = TreeNode::Node(value2, Box::new(new_left), right2);
                }
            },
            TreeNode::Leaf => {
                *self = TreeNode::Leaf;
            }
        }
    }
    /// Performs a right rotation on the tree
    pub fn right_rotate(&mut self) {
        let old_root = mem::take(self);
        match old_root {
            TreeNode::Node(value, left, right) => match *left {
                TreeNode::Leaf => {
                    *self = TreeNode::Node(value, Box::new(TreeNode::Leaf), right);
                }
                TreeNode::Node(value2, left2, right2) => {
                    let new_right = TreeNode::Node(value, right2, right);
                    *self = TreeNode::Node(value2, left2, Box::new(new_right));
                }
            },
            TreeNode::Leaf => {
                *self = TreeNode::Leaf;
            }
        }
    }

    /// Rebalances the tree using either a single or double rotation, as specified in the AVL tree
    /// rebalancing algorithm.
    fn rebalance(&mut self) {
        let balance = self.balance_factor();
        match self {
            TreeNode::Leaf => {}
            TreeNode::Node(_, left, right) => {
                if balance > 1 {
                    if left.balance_factor() < 0 {
                        left.left_rotate();
                    }
                    self.right_rotate();
                } else if balance < -1 {
                    if right.balance_factor() > 0 {
                        right.right_rotate();
                    }
                    self.left_rotate();
                }
            }
        }
    }
}

// Implement `Default` for `TreeNode<T>`
impl<T: Ord> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode::Leaf
    }
}

// Implement `PartialEq` for `TreeNode<T>`
impl<T: Ord> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TreeNode::Leaf, TreeNode::Leaf) => true,
            (TreeNode::Node(k, j, m), TreeNode::Node(k2, j2, m2)) => k == k2 && j == j2 && m == m2,
            _ => false,
        }
    }
}

// Implement `Eq` for `TreeNode<T>`
impl<T: Ord> Eq for TreeNode<T> {}

// Implement `From<Vec<T>>` for `TreeNode<T>`
impl<T: Ord> From<Vec<T>> for TreeNode<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut tree = TreeNode::Leaf;
        for value in vec {
            tree.insert(value);
        }
        tree
    }
}

// Implement `From<TreeNode<T>>` for `Vec<T>`
impl<T: Ord> From<TreeNode<T>> for Vec<T> {
    fn from(tree: TreeNode<T>) -> Self {
        let mut vec = Vec::new();
        fn inorder<T: Ord>(node: TreeNode<T>, vec: &mut Vec<T>) {
            match node {
                TreeNode::Leaf => {}
                TreeNode::Node(value, left, right) => {
                    inorder(*left, vec);
                    vec.push(value);
                    inorder(*right, vec);
                }
            }
        }
        inorder(tree, &mut vec);
        vec
    }
}
