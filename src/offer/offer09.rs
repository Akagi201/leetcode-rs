/// 剑指 Offer 09. 用两个栈实现队列
/// <https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/>

struct CQueue {
	s1: Vec<i32>,
	s2: Vec<i32>,
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
#[allow(unused)]
impl CQueue {
	fn new() -> Self {
		CQueue { s1: Vec::new(), s2: Vec::new() }
	}

	fn append_tail(&mut self, value: i32) {
		self.s1.push(value);
	}

	fn delete_head(&mut self) -> i32 {
		if self.s2.is_empty() {
			while !self.s1.is_empty() {
				self.s2.push(self.s1.pop().unwrap());
			}
		}
		self.s2.pop().unwrap_or(-1)
	}
}

/// Your CQueue object will be instantiated and called as such:
/// let obj = CQueue::new();
/// obj.append_tail(value);
/// let ret_2: i32 = obj.delete_head();
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut obj = CQueue::new();
		obj.append_tail(32);
		assert_eq!(32, obj.delete_head());
		assert_eq!(-1, obj.delete_head());

		obj.append_tail(3);
		assert_eq!(3, obj.delete_head());
		assert_eq!(-1, obj.delete_head());

		obj.append_tail(5);
		obj.append_tail(2);
		assert_eq!(5, obj.delete_head());
		assert_eq!(2, obj.delete_head());
	}
}
