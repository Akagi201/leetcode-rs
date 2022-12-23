// https://leetcode.cn/problems/li-wu-de-zui-da-jie-zhi-lcof/

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if i == 0 && j == 0 {
                    dp[i][j] = grid[i][j];
                } else if i == 0 {
                    dp[i][j] = dp[i][j - 1] + grid[i][j];
                } else if j == 0 {
                    dp[i][j] = dp[i - 1][j] + grid[i][j];
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
                }
            }
        }
        dp[grid.len() - 1][grid[0].len() - 1]
    }
}

#[test]
fn tests() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(Solution::max_value(grid), 12);
}
