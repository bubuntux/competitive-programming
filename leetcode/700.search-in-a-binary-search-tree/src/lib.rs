use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn search_bst(root: Option<Node>, target: i32) -> Option<Node> {
        Self::dfs(root.as_ref(), target)
    }

    fn dfs(root: Option<&Node>, target: i32) -> Option<Node> {
        match root {
            None => None,
            Some(node) => {
                let val = node.borrow().val;
                if target == val {
                    return Some(node.clone());
                }
                let right = Self::dfs(node.borrow().right.as_ref(), target);
                if right.is_some() {
                    return right;
                }
                Self::dfs(node.borrow().left.as_ref(), target)
            }
        }
    }
}

// https://leetcode.com/problems/search-in-a-binary-search-tree

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
        //todo fix
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
            Solution::search_bst(into_tree(vec![4, 2, 7, 1, 3]), 2),
            into_tree(vec![2, 1, 3])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::search_bst(into_tree(vec![4, 2, 7, 1, 3]), 5),
            into_tree(vec![])
        );
    }
}
