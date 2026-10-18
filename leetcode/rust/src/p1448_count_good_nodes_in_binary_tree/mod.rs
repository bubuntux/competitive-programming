use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::gt_x(root, None)
    }

    fn gt_x(root: Option<Rc<RefCell<TreeNode>>>, max: Option<i32>) -> i32 {
        let mut sum = 0;
        if let Some(node) = root {
            let current_val = node.borrow().val;
            let max = max.unwrap_or(current_val);
            if current_val >= max {
                sum += 1;
            }

            let max = max.max(current_val);
            sum += Solution::gt_x(node.borrow().left.clone(), Some(max));
            sum += Solution::gt_x(node.borrow().right.clone(), Some(max));
        }
        sum
    }
}

// https://leetcode.com/problems/count-good-nodes-in-binary-tree/

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
        //todo bug
        fn build_tree(v: &Vec<i32>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if i >= v.len() || v[i] == i32::MAX {
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
            Solution::good_nodes(into_tree(vec![3, 1, 4, 3, i32::MAX, 1, 5]),),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::good_nodes(into_tree(vec![3, 3, i32::MAX, 4, 2]),),
            3
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::good_nodes(into_tree(vec![1]),), 1);
    }

    #[test]
    fn example_11() {
        assert_eq!(
            Solution::good_nodes(into_tree(vec![
                2,
                i32::MAX,
                4,
                10,
                8,
                i32::MAX,
                i32::MAX,
                4
            ]),),
            4
        );
    }
}
