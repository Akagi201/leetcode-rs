/// 剑指 Offer 10- II. 青蛙跳台阶问题
/// <https://leetcode.cn/problems/qing-wa-tiao-tai-jie-wen-ti-lcof/>

pub struct Solution;

#[allow(unused)]
impl Solution {
	pub fn num_ways(n: i32) -> i32 {
		if n == 0 || n == 1 {
			return 1
		}
		let mut dp = vec![0; n as usize + 1];
		dp[0] = 1;
		dp[1] = 1;
		for i in 2..=n {
			let i = i as usize;
			dp[i] = (dp[i - 1] + dp[i - 2]) % 1000000007;
		}
		dp[n as usize]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::num_ways(2), 2);
	}
}
