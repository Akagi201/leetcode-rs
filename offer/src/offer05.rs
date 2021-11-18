// https://leetcode-cn.com/problems/ti-huan-kong-ge-lcof/
struct Solution;

#[allow(unused)]
impl Solution {
  pub fn replace_space(s: String) -> String {
    // s.replace(" ", "%20")
    s.chars().map(|c| if c == ' ' { "%20".to_string() } else { c.to_string() }).collect::<String>()
  }
}

#[test]
fn tests() {
  assert_eq!(Solution::replace_space("We are happy.".to_string()), "We%20are%20happy.".to_string());
}