/// 剑指 Offer 05. 替换空格
/// <https://leetcode.cn/problems/ti-huan-kong-ge-lcof/>

pub struct Solution;

impl Solution {
	pub fn replace_space(s: String) -> String {
		// s.replace(" ", "%20")
		s.chars()
			.map(|c| if c == ' ' { "%20".to_string() } else { c.to_string() })
			.collect::<String>()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(
			Solution::replace_space("We are happy.".to_string()),
			"We%20are%20happy.".to_string()
		);
	}
}
