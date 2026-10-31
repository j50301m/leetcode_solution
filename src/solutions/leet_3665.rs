struct Solution {}

const MOD: i32 = 1000000007;
impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![[0; 2]; n]; m];
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    dp[i][j][0] = 1; // 0 from left
                    dp[i][j][1] = 1; // 1 from top
                    continue;
                }
                let mut top = 0;
                let mut left = 0;
                if grid[i][j] == 1 {
                    if j < n - 1 && i > 0 {
                        if grid[i - 1][j] == 1 {
                            top = dp[i - 1][j][0];
                        } else {
                            top = dp[i - 1][j][1];
                        }
                    }
                    if i < m - 1 && j > 0 {
                        if grid[i][j - 1] == 1 {
                            left = dp[i][j - 1][1];
                        } else {
                            left = dp[i][j - 1][0];
                        }
                    }
                    dp[i][j][0] = left;
                    dp[i][j][1] = top;
                } else {
                    if i > 0 {
                        if grid[i - 1][j] == 1 {
                            top = dp[i - 1][j][0];
                        } else {
                            top = dp[i - 1][j][1];
                        }
                    }
                    if j > 0 {
                        if grid[i][j - 1] == 1 {
                            left = dp[i][j - 1][1];
                        } else {
                            left = dp[i][j - 1][0];
                        }
                    }
                    dp[i][j][0] = (top + left) % MOD;
                    dp[i][j][1] = (top + left) % MOD;
                }
            }
        }

        dp[m - 1][n - 1][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::unique_paths(vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            5
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::unique_paths(vec![vec![0, 0], vec![0, 0]]), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::unique_paths(vec![vec![0, 1, 1], vec![1, 1, 0]]),
            1
        );
    }
}
