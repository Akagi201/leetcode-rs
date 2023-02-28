/// 剑指 Offer 27. 二叉树的镜像
/// <https://leetcode.cn/problems/er-cha-shu-de-jing-xiang-lcof/>
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

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
		TreeNode { val, left: None, right: None }
	}
}

#[allow(unused)]
impl Solution {
	pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		root.as_ref()?;
		let mut root = root.unwrap();
		let mut left = root.borrow_mut().left.take();
		let mut right = root.borrow_mut().right.take();
		root.borrow_mut().left = Solution::mirror_tree(right);
		root.borrow_mut().right = Solution::mirror_tree(left);
		Some(root)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::mirror_tree(None), None);
		assert_eq!(
			Solution::mirror_tree(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
			Some(Rc::new(RefCell::new(TreeNode::new(1))))
		);
	}
}
