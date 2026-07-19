struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; n];
        dp[0] = cost[0];
        dp[1] = cost[1];

        for i in 2..n {
            let curr = cost[i];
            dp[i] = (dp[i - 1] + curr).min(dp[i - 2] + curr);
        }

        dp[n - 1].min(dp[n - 2])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![10, 15, 20];
        let result = Solution::min_cost_climbing_stairs(nums);
        assert_eq!(result, 15);
    }

    #[test]
    fn case_2() {
        let nums = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let result = Solution::min_cost_climbing_stairs(nums);
        assert_eq!(result, 6);
    }
}
