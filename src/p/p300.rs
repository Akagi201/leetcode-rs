// https://leetcode.cn/problems/longest-increasing-subsequence/
struct Solution;

impl Solution {
	fn length_of_lis(nums: Vec<i32>) -> i32 {
		let mut dp = vec![1; nums.len()];
		let mut res = 0;
		for i in 0..nums.len() {
			for j in 0..i {
				if nums[i] > nums[j] {
					dp[i] = std::cmp::max(dp[i], dp[j] + 1);
				}
			}
			res = std::cmp::max(res, dp[i]);
		}
		res
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
		let res = 4;
		assert_eq!(Solution::length_of_lis(nums), res);
	}
}
