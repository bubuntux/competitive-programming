use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn right_side_view(root: Option<Node>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::bfs(root.as_ref(), 0, &mut res);
        res
    }

    fn bfs(root: Option<&Node>, level: usize, path: &mut Vec<i32>) {
        match root {
            None => {}
            Some(node) => {
                if level == path.len() {
                    path.push(node.borrow().val)
                }
                Self::bfs(node.borrow().right.as_ref(), level + 1, path);
                Self::bfs(node.borrow().left.as_ref(), level + 1, path)
            }
        }
    }
}

// https://leetcode.com/problems/binary-tree-right-side-view/

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
            Solution::right_side_view(into_tree(vec![1, 2, 3, i32::MAX, 5, i32::MAX, 4]),),
            vec![1, 3, 4]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::right_side_view(into_tree(vec![1, i32::MAX, 3]),),
            vec![1, 3]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::right_side_view(into_tree(vec![]),), vec![]);
    }

    #[test]
    fn example_37() {
        assert_eq!(
            Solution::right_side_view(into_tree(vec![1, 2]),),
            vec![1, 2]
        );
    }

    #[test]
    fn example_158() {
        assert_eq!(
            Solution::right_side_view(into_tree(vec![1, 2, 3, 4]),),
            vec![1, 3, 4]
        );
    }
}
