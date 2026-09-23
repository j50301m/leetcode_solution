struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }

        let mut dp = vec![0; n + 1];
        dp[1] = nums[0];
        for i in 1..n {
            dp[i + 1] = dp[i].max(dp[i - 1] + nums[i]);
        }

        dp[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
