// https://leetcode.cn/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>,
}

impl ListNode {
	#[inline]
	fn new(val: i32) -> Self {
		ListNode { next: None, val }
	}
}

impl Solution {
	// pub fn reverse_print(head: Option<Box<list_node::ListNode>>) -> Vec<i32> leetcode
	// 上提交需要改成这行
	pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
		let mut result = vec![];
		let mut node = head;
		while let Some(n) = node {
			result.push(n.val);
			node = n.next;
		}
		result.reverse();
		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::reverse_print(None), vec![]);
		assert_eq!(
			Solution::reverse_print(Some(Box::new(ListNode { val: 2, next: None }))),
			vec![2]
		);
		assert_eq!(
			Solution::reverse_print(Some(Box::new(ListNode {
				val: 1,
				next: Some(Box::new(ListNode {
					val: 3,
					next: Some(Box::new(ListNode { val: 2, next: None })),
				})),
			}))),
			vec![2, 3, 1]
		);
	}
}
