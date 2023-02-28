// https://leetcode.cn/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof/
struct Solution;

#[allow(unused)]
impl Solution {
	pub fn reverse_left_words(s: String, n: i32) -> String {
		String::from(&s[n as usize..]) + &s[0..n as usize]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::reverse_left_words("abcdefg".to_string(), 2), "cdefgab");
	}
}
