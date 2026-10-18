use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut counter: i32 = 0;
        Solution::dfs(root.as_ref(), &mut counter);
        counter
    }

    pub fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, counter: &mut i32) -> i32 {
        if node.is_none() {
            return 0;
        }
        let n = node.unwrap().borrow();
        let left = Solution::dfs(n.left.as_ref(), counter);
        let right = Solution::dfs(n.right.as_ref(), counter);
        *counter += left.abs() + right.abs();
        n.val + left + right - 1
    }
}

// https://leetcode.com/problems/distribute-coins-in-binary-tree/

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut root = TreeNode::new(3);
        root.left = Option::Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.right = Option::Some(Rc::new(RefCell::new(TreeNode::new(0))));

        assert_eq!(
            2,
            Solution::distribute_coins(Option::Some(Rc::new(RefCell::new(root))))
        );
    }

    #[test]
    fn case_2() {
        let mut root = TreeNode::new(0);
        root.left = Option::Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.right = Option::Some(Rc::new(RefCell::new(TreeNode::new(0))));

        assert_eq!(
            3,
            Solution::distribute_coins(Option::Some(Rc::new(RefCell::new(root))))
        );
    }

    #[test]
    fn case_3() {
        let mut root = TreeNode::new(1);
        root.left = Option::Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.right = Option::Some(Rc::new(RefCell::new(TreeNode::new(2))));

        assert_eq!(
            2,
            Solution::distribute_coins(Option::Some(Rc::new(RefCell::new(root))))
        );
    }

    #[test]
    fn case_4() {
        let mut root = TreeNode::new(1);
        let mut left = TreeNode::new(0);
        left.left = Option::Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.left = Option::Some(Rc::new(RefCell::new(left)));
        root.right = Option::Some(Rc::new(RefCell::new(TreeNode::new(0))));

        assert_eq!(
            4,
            Solution::distribute_coins(Option::Some(Rc::new(RefCell::new(root))))
        );
    }
}
