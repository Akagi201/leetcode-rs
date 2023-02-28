/// 509. 斐波那契数
/// <https://leetcode.cn/problems/fibonacci-number/>
pub struct Solution;

impl Solution {
	// simple recursive
	// pub fn fib(n: i32) -> i32 {
	//     if n < 2 {
	//         return n;
	//     }
	//     Self::fib(n - 1) + Self::fib(n - 2)
	// }

	// Warning: This will overflow the 128-bit unsigned integer at n=186
	pub fn fib(n: i32) -> i32 {
		// Use a and b to store the previous two values in the sequence
		let mut a = 0;
		let mut b = 1;
		for _i in 0..n {
			// As we iterate through, move b's value into a and the new computed
			// value into b.
			let c = a + b;
			a = b;
			b = c;
		}
		a
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::fib(0), 0);
		assert_eq!(Solution::fib(1), 1);
		assert_eq!(Solution::fib(2), 1);
		assert_eq!(Solution::fib(3), 2);
		assert_eq!(Solution::fib(4), 3);
		assert_eq!(Solution::fib(5), 5);
		assert_eq!(Solution::fib(6), 8);
	}
}
