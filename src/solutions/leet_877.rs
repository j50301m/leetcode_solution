struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            for j in i..n {
                if i == j {
                    dp[i][j] = piles[i];
                    continue;
                }

                let left = piles[i] - dp[i + 1][j];
                let right = piles[j] - dp[i][j - 1];
                dp[i][j] = left.max(right);
            }
        }

        dp[0][n - 1] > 0
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let piles = vec![5, 3, 4, 5];
        let result = Solution::stone_game(piles);
        assert_eq!(result, true);
    }
}
