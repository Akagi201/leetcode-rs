// https://leetcode.cn/problems/shu-de-zi-jie-gou-lcof/

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
    pub fn is_sub_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if b.is_none() {
            return false;
        }
        Self::sub_structure(a, b)
    }
    // 遍历树 A 的所有结点，对 A 的每一个节点调用 equal_structure 函数
    pub fn sub_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let temp = Self::equal_structure(a.clone(), b.clone());
        match a {
            Some(mut node) => {
                let left_tree = node.borrow_mut().left.take();
                let right_tree = node.borrow_mut().right.take();
                temp || Self::sub_structure(left_tree, b.clone())
                    || Self::sub_structure(right_tree, b)
            }
            None => false,
        }
    }

    // 判断以树 A 的某一个子节点为根节点的子树是否包含树 B
    pub fn equal_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (Some(node1), Some(node2)) => {
                let main_val = node1.borrow().val;
                let branch_val = node2.borrow().val;
                main_val == branch_val
                    && Self::equal_structure(
                        node1.borrow().left.clone(),
                        node2.borrow().left.clone(),
                    )
                    && Self::equal_structure(
                        node1.borrow().right.clone(),
                        node2.borrow().right.clone(),
                    )
            }
            (Some(node1), None) => true,
            (None, Some(node2)) => false,
            (None, None) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_sub_structure(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        ));
    }
}
