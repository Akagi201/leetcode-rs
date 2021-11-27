// https://leetcode-cn.com/problems/lian-xu-zi-shu-zu-de-zui-da-he-lcof/

struct Solution;

#[allow(unused)]
// impl Solution {
//   pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//       nums.into_iter().fold((0, i32::MIN), |(mut s, ans), x| {
//           s = max(s + x, x);
//           (s, max(ans, s))
//       }).1
//   }
// }

// DP 不优化 68 ms, 2.8MB
// impl Solution {
//   pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//     if nums.is_empty() {
//       return 0;
//     } else if nums.len() == 1 {
//       return nums[0];
//     }
//     let mut dp = vec![0; nums.len()];
//     let mut res = i32::MIN;
//     for i in 0..nums.len() {
//       if i == 0 {
//         dp[i] = nums[i];
//         res = dp[i];
//       } else {
//         println!("i:{}, dp[i-1]:{}, nums[i]:{}", i, dp[i-1], nums[i]);
//         dp[i] = max(dp[i - 1] + nums[i], nums[i]);
//         res = max(res, dp[i]);
//       }
//     }
//     res
//   }
// }

// DP 空间优化 4 ms, 2.7 MB
impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
      return 0;
    }
    let (mut res, mut pre) = (i32::MIN, 0);
    for n in nums {
      pre = n.max(pre+n);
      res = res.max(pre);
    }
    res
  }
}

#[test]
fn tests() {
  assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
  assert_eq!(Solution::max_sub_array(vec![-2,1]), 1);
}