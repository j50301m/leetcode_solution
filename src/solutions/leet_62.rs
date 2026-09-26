struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![1; n]; m];

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
