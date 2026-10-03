struct Solution {}

impl Solution {
    pub fn minimum_costs(regular: Vec<i32>, express: Vec<i32>, express_cost: i32) -> Vec<i64> {
        let n = regular.len();
        let mut reg = vec![0i64; n + 1];
        let mut exp = vec![0i64; n + 1];
        let mut result = vec![0i64; n];
        exp[0] = express_cost as i64;

        for i in 1..=n {
            let r_cost = regular[i - 1] as i64;
            let e_cost = express[i - 1] as i64;
            reg[i] = (reg[i - 1] + r_cost).min(exp[i - 1] + r_cost);
            exp[i] = (exp[i - 1] + e_cost).min(reg[i - 1] + e_cost + express_cost as i64);

            result[i - 1] = reg[i].min(exp[i]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::minimum_costs(vec![1, 6, 9, 5], vec![5, 2, 3, 10], 8),
            vec![1, 7, 14, 19]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::minimum_costs(vec![11, 5, 13], vec![7, 10, 6], 3),
            vec![10, 15, 24]
        );
    }

    #[test]
    fn single_stop() {
        assert_eq!(Solution::minimum_costs(vec![5], vec![1], 10), vec![5]);
    }

    #[test]
    fn switch_back_to_regular_is_free() {
        // 普通 -> 快線 -> 普通，回普通線不用付錢
        assert_eq!(
            Solution::minimum_costs(vec![1, 100, 1], vec![100, 1, 100], 1),
            vec![1, 3, 4]
        );
    }

    #[test]
    fn large_sum_exceeds_i32() {
        let n = 100_000;
        let result = Solution::minimum_costs(vec![100_000; n], vec![100_000; n], 100_000);
        assert_eq!(result[n - 1], 10_000_000_000);
    }
}
