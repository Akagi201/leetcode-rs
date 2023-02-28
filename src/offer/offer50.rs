/// 剑指 Offer 50. 第一个只出现一次的字符
/// <https://leetcode.cn/problems/di-yi-ge-zhi-chu-xian-yi-ci-de-zi-fu-lcof/>

pub struct Solution;

#[allow(unused)]
impl Solution {
	pub fn first_uniq_char(s: String) -> char {
		let mut map = std::collections::HashMap::new();
		for c in s.chars() {
			*map.entry(c).or_insert(0) += 1;
		}
		for c in s.chars() {
			if map[&c] == 1 {
				return c
			}
		}
		' '
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 'l');
		assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 'v');
	}
}
