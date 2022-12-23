// https://leetcode.cn/problems/xuan-zhuan-shu-zu-de-zui-xiao-shu-zi-lcof/
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        numbers.into_iter().min().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_array(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::min_array(vec![3, 3, 1, 3]), 1);
        assert_eq!(Solution::min_array(vec![1, 3, 3]), 1);
    }
}
