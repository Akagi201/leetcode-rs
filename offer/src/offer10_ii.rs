// https://leetcode-cn.com/problems/qing-wa-tiao-tai-jie-wen-ti-lcof/

struct Solution;

#[allow(unused)]
impl Solution {
  pub fn num_ways(n: i32) -> i32 {
    if n == 0 {
      return 1;
    } else if n == 1 {
      return 1;
    }
    let mut dp = vec![0; n as usize + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
      let i = i as usize;
      dp[i] = (dp[i - 1] + dp[i - 2])%1000000007;
    }
    dp[n as usize]
  }
}

#[test]
fn tests() {
  assert_eq!(Solution::num_ways(2), 2);
}
