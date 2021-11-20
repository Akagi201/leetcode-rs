// https://leetcode-cn.com/problems/di-yi-ge-zhi-chu-xian-yi-ci-de-zi-fu-lcof/

struct Solution;

#[allow(unused)]
impl Solution {
  pub fn first_uniq_char(s: String) -> char {
    let mut map = std::collections::HashMap::new();
    for c in s.chars() {
      *map.entry(c).or_insert(0) += 1;
    }
    for c in s.chars() {
      if map[&c] == 1 {
        return c;
      }
    }
    ' '
  }
}

#[test]
fn tests() {
  assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 'l');
  assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 'v');
}