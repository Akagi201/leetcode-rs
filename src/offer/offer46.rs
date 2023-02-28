// https://leetcode.cn/problems/ba-shu-zi-fan-yi-cheng-zi-fu-chuan-lcof/

struct Solution;

#[allow(unused)]
impl Solution {
	pub fn translate_num(num: i32) -> i32 {
		const RADIX: u32 = 10;
		let mut nums =
			num.to_string().chars().flat_map(|c| c.to_digit(RADIX)).collect::<Vec<u32>>();
		let count = nums.len();
		// dp[i] 表示 num[0..i] 的翻译结果的个数
		// 状态转移方程：dp[i] = dp[i - 1] + dp[i - 2](如果 num[(i-1)..i] 可以翻译成一个数字)
		let mut dp = vec![0; count];
		dp[0] = 1; // 0 - 9 可以翻译成 1 种结果
		for i in 1..count {
			dp[i] = dp[i - 1]; // 最后一位一定可以翻译
			let last2num = nums[i - 1] * 10 + nums[i];
			if (10..=25).contains(&last2num) {
				if i >= 2 {
					dp[i] += dp[i - 2]; // 前两位可以翻译成一个数字
				} else {
					dp[i] += 1;
				}
			}
		}
		dp[count - 1]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::translate_num(123), 3);
	}
}
