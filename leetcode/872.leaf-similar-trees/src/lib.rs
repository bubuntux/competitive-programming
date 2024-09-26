use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut v1: Vec<i32> = Vec::new();
        Self::add_leafs(root1, &mut v1);

        let mut v2: Vec<i32> = Vec::new();
        Self::add_leafs(root2, &mut v2);

        v1.eq(&v2)
    }

    fn add_leafs(root: Option<Rc<RefCell<TreeNode>>>, mut v: &mut Vec<i32>) {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                v.push(node.borrow().val);
            } else {
                Solution::add_leafs(node.borrow().left.clone(), &mut v);
                Solution::add_leafs(node.borrow().right.clone(), &mut v);
            }
        }
    }
}

// https://leetcode.com/problems/leaf-similar-trees/

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
            Solution::leaf_similar(
                into_tree(vec![3, 5, 1, 6, 2, 9, 8, 999, 999, 7, 4]),
                into_tree(vec![
                    3, 5, 1, 6, 7, 4, 2, 999, 999, 999, 999, 999, 999, 9, 8
                ])
            ),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::leaf_similar(into_tree(vec![1, 2, 3]), into_tree(vec![1, 3, 2])),
            false
        );
    }
}
