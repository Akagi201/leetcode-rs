/// 剑指 Offer 63. 股票的最大利润
/// <https://leetcode.cn/problems/gu-piao-de-zui-da-li-run-lcof/>

pub struct Solution;

#[allow(unused)]
impl Solution {
	pub fn max_profit(prices: Vec<i32>) -> i32 {
		let mut min_price = i32::MAX;
		let mut max_profit = 0;
		for price in prices {
			min_price = min_price.min(price);
			max_profit = max_profit.max(price - min_price);
		}
		max_profit
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
		assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
		assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
	}
}
