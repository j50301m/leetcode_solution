pub struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = piles.len();
        let mut dp = vec![vec![0; (k + 1) as usize]; n + 1];

        for i in 1..=n {
            let pile = &piles[i - 1];

            for j in 0..=k as usize {
                dp[i][j] = dp[i - 1][j];

                let mut prefix = 0;
                for t in 1..=pile.len().min(j) {
                    prefix += pile[t - 1];

                    dp[i][j] = dp[i][j].max(dp[i - 1][j - t] + prefix);
                }
            }
        }

        dp[n][k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let piles = vec![vec![1, 100, 3], vec![7, 8, 9]];
        let result = Solution::max_value_of_coins(piles, 2);
        assert_eq!(result, 101);
    }

    #[test]
    fn case_2() {
        let piles = vec![
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![100],
            vec![1, 1, 1, 1, 1, 1, 700],
        ];
        let result = Solution::max_value_of_coins(piles, 7);
        assert_eq!(result, 706);
    }
}
