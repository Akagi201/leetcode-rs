// https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof/

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if root.is_none() {
            return res;
        }
        let mut queue = vec![root.unwrap()]; // queue 表示一层的节点
        let mut height = 0;
        while !queue.is_empty() {
            height += 1;
            let mut tmp = vec![]; // 下一层节点
            let mut tmp_res = std::collections::VecDeque::new(); // 每一层输出内容
            for node in queue {
                let n1 = node.borrow();
                if height % 2 == 1 {
                    tmp_res.push_back(n1.val);
                } else {
                    tmp_res.push_front(n1.val);
                }
                if n1.left.is_some() {
                    tmp.push(n1.left.clone().unwrap());
                }
                if n1.right.is_some() {
                    tmp.push(n1.right.clone().unwrap());
                }
            }
            let this_res = tmp_res.into_iter().collect::<Vec<i32>>();
            res.push(this_res);
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
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(
            Solution::level_order(root),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}
