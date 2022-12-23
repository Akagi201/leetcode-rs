// https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
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

#[allow(unused)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }
        let mut queue = vec![root.unwrap()]; // queue 表示一层的节点
        while !queue.is_empty() {
            let mut tmp = vec![];
            for node in queue {
                let n1 = node.borrow();
                res.push(n1.val);
                if n1.left.is_some() {
                    tmp.push(n1.left.clone().unwrap());
                }
                if n1.right.is_some() {
                    tmp.push(n1.right.clone().unwrap());
                }
            }
            queue = tmp;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::level_order(None), vec![]);
        assert_eq!(
            Solution::level_order(Some(Rc::new(RefCell::new(TreeNode::new(3)))),),
            vec![3]
        );
    }
}
