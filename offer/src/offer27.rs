// https://leetcode-cn.com/problems/er-cha-shu-de-jing-xiang-lcof/

use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

#[allow(unused)]
impl Solution {
    pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mut root = root.unwrap();
        let mut left = root.borrow_mut().left.take();
        let mut right = root.borrow_mut().right.take();
        root.borrow_mut().left = Solution::mirror_tree(right);
        root.borrow_mut().right = Solution::mirror_tree(left);
        Some(root)
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::mirror_tree(None), None);
    assert_eq!(Solution::mirror_tree(Some(Rc::new(RefCell::new(TreeNode::new(1))))), Some(Rc::new(RefCell::new(TreeNode::new(1)))));
}