// https://leetcode-cn.com/problems/coin-change/

struct Solution;

#[allow(unused)]
impl Solution {
  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // dp[i] = min(dp[i], dp[i - coins[j]] + 1), dp[i] 表示组成面额 i 的最少硬币数
    let mut dp = vec![amount + 1; amount as usize + 1];
    dp[0] = 0;
    for i in 1..=amount {
      let mut min = amount + 1;
      for &coin in coins.iter() {
        if i >= coin {
          min = min.min(dp[(i - coin) as usize]);
        }
      }
      dp[i as usize] = min + 1;
    }
    if dp[amount as usize] > amount {
      -1
    } else {
      dp[amount as usize]
    }
  }
}

#[test]
fn tests() {
  assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
  assert_eq!(Solution::coin_change(vec![2], 3), -1);
  assert_eq!(Solution::coin_change(vec![1, 2, 5], 100), 20);
}