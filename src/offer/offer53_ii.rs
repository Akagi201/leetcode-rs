#![allow(clippy::needless_range_loop)]
/// 剑指 Offer 53 - II. 0～n-1 中缺失的数字
/// <https://leetcode.cn/problems/que-shi-de-shu-zi-lcof/>
pub struct Solution;

#[allow(unused)]
impl Solution {
	pub fn missing_number(nums: Vec<i32>) -> i32 {
		let mut sum = 0;
		for i in 0..nums.len() {
			sum += nums[i];
		}
		let mut sum_2 = (nums.len() as i32) * (nums.len() as i32 + 1) / 2;
		sum_2 -= sum;
		sum_2
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
		assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
	}
}
