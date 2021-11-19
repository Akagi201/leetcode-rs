// https://leetcode-cn.com/problems/er-wei-shu-zu-zhong-de-cha-zhao-lcof/

struct Solution;

#[allow(unused)]
impl Solution {
  pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    #[warn(clippy::search_is_some)]
    matrix.iter().find(|row| row.binary_search(&target).is_ok()).is_some()
  }
}

#[test]
fn tests() {
  let matrix = vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]];
  assert!(Solution::find_number_in2_d_array(matrix, 5));
  // assert_eq!(Solution::find_number_in2_d_array(matrix, 20), false);
}