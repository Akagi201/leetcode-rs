// https://leetcode.cn/problems/fei-bo-na-qi-shu-lie-lcof/

struct Solution;

#[allow(unused)]
impl Solution {
	pub fn fib(n: i32) -> i32 {
		if n == 0 || n == 1 {
			return n
		}
		let mut a = 0;
		let mut b = 1;
		for _ in 2..n + 1 {
			let c = (a + b) % 1000000007;
			a = b;
			b = c;
		}
		b
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::fib(2), 1);
		assert_eq!(Solution::fib(3), 2);
		assert_eq!(Solution::fib(4), 3);
		assert_eq!(Solution::fib(45), 134903163);
	}
}
