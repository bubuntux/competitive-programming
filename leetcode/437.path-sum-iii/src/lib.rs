use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn path_sum(root: Option<Node>, target_sum: i32) -> i32 {
        Self::dfs(root.as_ref(), &mut vec![], target_sum as i64)
    }

    fn dfs(node: Option<&Node>, sums: &mut Vec<i64>, target_sum: i64) -> i32 {
        if node.is_none() {
            return 0;
        }

        let val = node.unwrap().borrow().val as i64;
        sums.iter_mut().for_each(|sum| *sum += val);
        sums.push(val);
        let mut paths = sums.iter().filter(|&sum| sum == &target_sum).count() as i32;
        paths += Self::dfs(node.unwrap().borrow().left.as_ref(), sums, target_sum);
        paths += Self::dfs(node.unwrap().borrow().right.as_ref(), sums, target_sum);
        sums.pop();
        sums.iter_mut().for_each(|sum| *sum -= val);

        paths
    }
}

// https://leetcode.com/problems/path-sum-iii/

struct Solution;

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
            Solution::path_sum(
                into_tree(vec![10, 5, -3, 3, 2, i32::MAX, 11, 3, -2, i32::MAX, 1]),
                8
            ),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::path_sum(
                into_tree(vec![
                    5,
                    4,
                    8,
                    11,
                    i32::MAX,
                    13,
                    4,
                    7,
                    2,
                    i32::MAX,
                    i32::MAX,
                    5,
                    1
                ]),
                22
            ),
            3
        );
    }

    #[test]
    fn example_89() {
        assert_eq!(Solution::path_sum(into_tree(vec![1, 2]), 2), 1);
    }

    #[test]
    fn example_94() {
        assert_eq!(Solution::path_sum(into_tree(vec![1]), 1), 1);
    }

    #[test]
    fn example_100() {
        assert_eq!(Solution::path_sum(into_tree(vec![0, 1, 1]), 1), 4);
    }

    #[test]
    fn example_128() {
        assert_eq!(
            Solution::path_sum(
                into_tree(vec![
                    1000000000,
                    1000000000,
                    i32::MAX,
                    294967296,
                    i32::MAX,
                    1000000000,
                    i32::MAX,
                    1000000000,
                    i32::MAX,
                    1000000000
                ]),
                0
            ),
            0
        );
    }
}
