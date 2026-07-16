struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let coins: Vec<usize> = coins.into_iter().map(|c| c as usize).collect();
        let sentinel = amount as i32 + 1;
        let mut dp = vec![sentinel; amount + 1];
        dp[0] = 0;

        for a in 1..=amount {
            for &coin in &coins {
                if coin <= a {
                    dp[a] = dp[a].min(dp[a - coin] + 1);
                }
            }
        }

        if dp[amount] == sentinel {
            -1
        } else {
            dp[amount]
        }
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let coins = vec![1, 2, 5];
        let result = Solution::coin_change(coins, 11);
        assert_eq!(result, 3);
    }
}
