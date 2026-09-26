struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut free = vec![0; n];
        let mut hold = vec![0; n];
        hold[0] = -prices[0];
        for i in 1..n {
            free[i] = free[i - 1].max(hold[i - 1] + prices[i] - fee);
            hold[i] = hold[i - 1].max(free[i - 1] - prices[i]);
        }
        free[n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    }
}
