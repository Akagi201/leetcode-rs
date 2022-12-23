// https://leetcode.cn/problems/zai-pai-xu-shu-zu-zhong-cha-zhao-shu-zi-lcof/

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter().filter(|&&x| x == target).count() as i32
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::search(vec![], 0), 0);
    assert_eq!(Solution::search(vec![1], 1), 1);
    assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 8), 2);
    assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 6), 0);
}
