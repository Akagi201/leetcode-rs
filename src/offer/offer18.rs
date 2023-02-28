// https://leetcode.cn/problems/shan-chu-lian-biao-de-jie-dian-lcof/
use crate::ListNode;

struct Solution;

impl Solution {
	pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
		let mut dummy = Some(Box::new(ListNode { next: head, val: 0 }));
		let mut node = &mut dummy;
		while node.is_some() && node.as_ref().unwrap().next.is_some() {
			if node.as_ref().unwrap().next.as_ref().unwrap().val == val {
				let n = node.as_mut().unwrap().next.take();
				node.as_mut().unwrap().next = n.unwrap().next;
				break
			}
			node = &mut node.as_mut().unwrap().next;
		}
		dummy.unwrap().next
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::list;

	#[test]
	fn test() {
		let cases = vec![
			(vec![4, 1, 9], (list![4, 5, 1, 9], 5)),
			(vec![4, 5, 9], (list![4, 5, 1, 9], 1)),
			(vec![], (list![], 1)),
			(vec![4, 1, 9], (list![4, 1, 9], 3)),
		];

		for (expect, (head, val)) in cases {
			assert_eq!(expect, ListNode::into_vec(Solution::delete_node(head, val)));
		}
	}
}
