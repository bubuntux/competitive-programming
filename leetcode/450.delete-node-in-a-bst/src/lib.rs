use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn delete_node(mut root: Option<Node>, target: i32) -> Option<Node> {
        match root {
            None => None,
            Some(node) => {
                let mut nb = node.borrow_mut();
                let val = nb.val;
                match val.cmp(&target) {
                    Ordering::Less => nb.right = Self::delete_node(nb.right.clone(), target),
                    Ordering::Equal => match (nb.left.as_ref(), nb.right.as_ref()) {
                        (None, None) => return None,
                        (Some(_), None) => return nb.left.take(),
                        (None, Some(_)) => return nb.right.take(),
                        (Some(_), Some(_)) => {
                            let min = Solution::find_min(nb.right.clone());
                            let min_val = min.unwrap().borrow().val;
                            nb.val = min_val;
                            nb.right = Solution::delete_node(nb.right.clone(), min_val)
                        }
                    },
                    Ordering::Greater => nb.left = Self::delete_node(nb.left.clone(), target),
                }
                Some(node.clone())
            }
        }
    }

    fn find_min(mut root: Option<Node>) -> Option<Node> {
        let mut temp = None;
        while root.as_ref().is_some() {
            temp = root.clone();
            root = root.unwrap().borrow_mut().left.clone();
        }
        temp.clone()
    }
}

// https://leetcode.com/problems/delete-node-in-a-bst

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
            Solution::delete_node(into_tree(vec![5, 3, 6, 2, 4, i32::MAX, 7]), 3),
            into_tree(vec![5, 4, 6, 2, i32::MAX, i32::MAX, 7])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::delete_node(into_tree(vec![5, 3, 6, 2, 4, i32::MAX, 7]), 0),
            into_tree(vec![5, 3, 6, 2, 4, i32::MAX, 7])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::delete_node(into_tree(vec![]), 0),
            into_tree(vec![])
        );
    }

    #[test]
    fn example_60() {
        assert_eq!(
            Solution::delete_node(into_tree(vec![5, 3, 6, 2, 4, i32::MAX, 7]), 5),
            into_tree(vec![6, 3, 7, 2, 4])
        );
    }
}
