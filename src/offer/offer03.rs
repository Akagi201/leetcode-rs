/// 剑指 Offer 03. 数组中重复的数字
/// <https://leetcode.cn/problems/shu-zu-zhong-zhong-fu-de-shu-zi-lcof/>
pub struct Solution;

#[allow(unused)]
impl Solution {
	pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
		if nums.is_empty() {
			return 0
		}
		let mut set = std::collections::HashSet::new();
		for &num in nums.iter() {
			if !set.insert(num) {
				return num
			}
		}
		nums[0]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]), 2);
		assert_eq!(Solution::find_repeat_number(vec![2, 2]), 2);
		assert_eq!(Solution::find_repeat_number(vec![2, 3]), 2);
	}
}
