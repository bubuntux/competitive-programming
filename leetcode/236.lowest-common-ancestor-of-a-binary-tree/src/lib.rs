use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Node>,
        p: Option<Node>,
        q: Option<Node>,
    ) -> Option<Node> {
        let aux = Self::dfs(root.as_ref(), p?.borrow().val, q?.borrow().val);
        if aux.0 == 2 {
            aux.1
        } else {
            None
        }
    }

    fn dfs(opt_node: Option<&Node>, p: i32, q: i32) -> (u8, Option<Node>) {
        match opt_node {
            None => (0, None),
            Some(node) => {
                let left = Self::dfs(node.borrow().left.as_ref(), p, q);
                if left.1.is_some() {
                    return left;
                }
                let right = Self::dfs(node.borrow().right.as_ref(), p, q);
                if right.1.is_some() {
                    return right;
                }
                let val = node.borrow().val;
                let mut finds = 0;
                if val == p || val == q {
                    finds += 1;
                }
                finds += left.0 + right.0;
                if finds == 2 {
                    (finds, Some(node.clone()))
                } else {
                    (finds, None)
                }
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
            Solution::lowest_common_ancestor(
                into_tree(vec![3, 5, 1, 6, 2, 0, 8, i32::MAX, i32::MAX, 7, 4]),
                Some(Node::new(RefCell::new(TreeNode::new(5)))),
                Some(Node::new(RefCell::new(TreeNode::new(1))))
            )
            .unwrap()
            .borrow()
            .val,
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                into_tree(vec![3, 5, 1, 6, 2, 0, 8, i32::MAX, i32::MAX, 7, 4]),
                Some(Node::new(RefCell::new(TreeNode::new(5)))),
                Some(Node::new(RefCell::new(TreeNode::new(4))))
            )
            .unwrap()
            .borrow()
            .val,
            5
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                into_tree(vec![1, 2]),
                Some(Node::new(RefCell::new(TreeNode::new(1)))),
                Some(Node::new(RefCell::new(TreeNode::new(2))))
            )
            .unwrap()
            .borrow()
            .val,
            1
        );
    }
}
