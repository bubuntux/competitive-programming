use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn max_level_sum(root: Option<Node>) -> i32 {
        let mut sums = Vec::new();
        Self::dfs(root.as_ref(), 0, &mut sums);
        sums.iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
            .map(|(i, _)| i)
            .unwrap_or(0) as i32
            + 1
    }

    fn dfs(root: Option<&Node>, level: usize, sums: &mut Vec<i32>) {
        match root {
            None => {}
            Some(node) => {
                let val = node.borrow().val;
                if let Some(x) = sums.get_mut(level) {
                    *x += val;
                } else {
                    sums.insert(level, val);
                }
                Self::dfs(node.borrow().right.as_ref(), level + 1, sums);
                Self::dfs(node.borrow().left.as_ref(), level + 1, sums)
            }
        }
    }
}

// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/

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
            Solution::max_level_sum(into_tree(vec![1, 7, 0, 7, -8, i32::MAX, i32::MAX])),
            2
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_level_sum(into_tree(vec![
                989,
                i32::MAX,
                10250,
                98693,
                -89388,
                i32::MAX,
                i32::MAX,
                i32::MAX,
                -32127
            ])),
            2
        );
    }
}
