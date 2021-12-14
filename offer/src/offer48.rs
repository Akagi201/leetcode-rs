// https://leetcode-cn.com/problems/zui-chang-bu-han-zhong-fu-zi-fu-de-zi-zi-fu-chuan-lcof/
use std::collections::HashMap;

struct Solution;

#[allow(unused)]
impl Solution {
  // 双指针 + hash
  // https://leetcode-cn.com/problems/zui-chang-bu-han-zhong-fu-zi-fu-de-zi-zi-fu-chuan-lcof/solution/mian-shi-ti-48-zui-chang-bu-han-zhong-fu-zi-fu-d-9/
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut ans = 0;
    let mut dict = HashMap::new();
    let len = s.len();
    let v = s.chars().collect::<Vec<char>>();
    let mut j = -1;

    for i in 0..len {
        if dict.contains_key(&v[i]) {
            j = (*dict.get(&v[i]).unwrap() as i32).max(j)
        }

        dict.insert(v[i], i);

        ans = ans.max(i as i32 - j);
    }

    ans
  }
}

#[test]
fn tests() {
  assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
  assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
  assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
  assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
  assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2);
  assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
  assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
  assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
  assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
  assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
  assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
  assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
  assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2);
  assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
  assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
  assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
  assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
}
