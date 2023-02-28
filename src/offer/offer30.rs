/// 剑指 Offer 30. 包含 min 函数的栈
/// <https://leetcode.cn/problems/bao-han-minhan-shu-de-zhan-lcof/>
use std::cmp::min;
struct MinStack {
	stack: Vec<(i32, Option<i32>)>, // (value，前一个 min)
	min: Option<i32>,               // 当前 min
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
#[allow(unused)]
impl MinStack {
	/// initialize your data structure here.
	fn new() -> Self {
		MinStack { stack: vec![], min: None }
	}

	fn push(&mut self, x: i32) {
		let min = match self.min {
			None => x,
			Some(m) => min(m, x),
		};
		self.stack.push((x, self.min));
		self.min = Some(min);
	}

	fn pop(&mut self) {
		if let Some(x) = self.stack.pop() {
			self.min = x.1;
		}
	}

	fn top(&self) -> i32 {
		self.stack.last().unwrap().0
	}

	fn min(&self) -> i32 {
		self.min.unwrap()
	}
}

/// Your MinStack object will be instantiated and called as such:
/// let obj = MinStack::new();
/// obj.push(x);
/// obj.pop();
/// let ret_3: i32 = obj.top();
/// let ret_4: i32 = obj.min();
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut obj = MinStack::new();
		obj.push(-2);
		obj.push(0);
		obj.push(-3);
		assert_eq!(obj.min(), -3);
		obj.pop();
		assert_eq!(obj.top(), 0);
		assert_eq!(obj.min(), -2);
	}
}
