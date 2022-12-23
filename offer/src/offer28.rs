// https://leetcode.cn/problems/dui-cheng-de-er-cha-shu-lcof/

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let root = root.unwrap();
        let mut left = root.borrow().left.clone();
        let mut right = root.borrow().right.clone();
        Solution::is_symmetric_helper(&left, &right)
    }

    pub fn is_symmetric_helper(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if left.is_none() || right.is_none() {
            return false;
        }
        let left = left.as_ref().unwrap().borrow();
        let right = right.as_ref().unwrap().borrow();
        if left.val != right.val {
            return false;
        }
        Solution::is_symmetric_helper(&left.left, &right.right)
            && Solution::is_symmetric_helper(&left.right, &right.left)
    }
}
