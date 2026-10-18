use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn longest_zig_zag(root: Option<Node>) -> i32 {
        Self::dfs(root.as_ref(), 0, 0)
    }

    fn dfs(opt_node: Option<&Node>, left_sum: u32, right_sum: u32) -> i32 {
        match opt_node {
            None => left_sum.max(right_sum) as i32 - 1,
            Some(node) => {
                let left = Self::dfs(node.borrow().left.as_ref(), 0, left_sum + 1);
                let right = Self::dfs(node.borrow().right.as_ref(), right_sum + 1, 0);
                left.max(right)
            }
        }
    }
}

// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
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

    fn into_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(v: &Vec<i32>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if i >= v.len() || v[i] > 100 || v[i] < -100 {
                return None;
            }
            let node = Rc::new(RefCell::new(TreeNode::new(v[i])));
            node.borrow_mut().left = build_tree(v, 2 * i + 1);
            node.borrow_mut().right = build_tree(v, 2 * i + 2);
            Some(node)
        }
        build_tree(&nums, 0)
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_zig_zag(into_tree(vec![
                1,
                i32::MAX,
                1,
                1,
                1,
                i32::MAX,
                i32::MAX,
                1,
                1,
                i32::MAX,
                1,
                i32::MAX,
                i32::MAX,
                i32::MAX,
                1
            ])),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_zig_zag(into_tree(vec![
                1,
                1,
                1,
                i32::MAX,
                1,
                i32::MAX,
                i32::MAX,
                1,
                1,
                i32::MAX,
                1
            ])),
            4
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::longest_zig_zag(into_tree(vec![1])), 0);
    }
}
