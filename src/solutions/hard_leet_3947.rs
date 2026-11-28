struct Solution {}

impl Solution {
    pub fn maximum_sale_items(mut items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let n = items.len();
        items.sort_by_key(|x| x[1]);
        let mut factor_count = vec![0; n + 1];
        for i in 0..n {
            let factor = items[i][0] as usize;
            factor_count[factor] += 1;
        }

        let mut multiple_count = vec![0; n + 1];
        for factor in 1..n + 1 {
            for multiple in (factor..n + 1).step_by(factor) {
                multiple_count[factor] += factor_count[multiple];
            }
        }

        let mut prize_count = vec![0; n];
        for i in 0..n {
            let factor = items[i][0] as usize;
            prize_count[i] = multiple_count[factor] - 1;
        }

        let min_price = items[0][1];
        let mut best = 0;
        let mut budget = budget;
        for i in 0..n {
            let price = items[i][1];
            if price > 2 * min_price {
                break;
            }
            let buy_count = prize_count[i].min(budget / price);
            budget -= buy_count * price;
            best += 2 * buy_count;
        }

        best += budget / min_price;
        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![1, 6], vec![2, 4], vec![3, 5]], 19),
            5
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::maximum_sale_items(
                vec![
                    vec![2, 8],
                    vec![1, 10],
                    vec![6, 6],
                    vec![4, 12],
                    vec![5, 20],
                    vec![5, 17]
                ],
                35
            ),
            7
        );
    }

    // 每個 price 都比 budget 大：一個都買不起
    #[test]
    fn cannot_afford() {
        assert_eq!(Solution::maximum_sale_items(vec![vec![1, 10]], 5), 0);
    }

    // price 10 > 2 * 1：花 10 拿 2 不如買 10 個 price 1
    #[test]
    fn deal_too_expensive() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![1, 10], vec![2, 1]], 10),
            10
        );
    }

    // budget 3、min_price 2：只買最便宜的只有 1 個，花 3 拿 2 比較多
    #[test]
    fn deal_beats_cheapest() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![2, 2], vec![1, 3]], 3),
            2
        );
    }

    // factor 相同：互相整除，各送 1 個
    #[test]
    fn same_factor() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![1, 5], vec![1, 5]], 10),
            4
        );
    }

    // price、budget 都是 10^9：2 * min_price = 2 * 10^9 不能溢位
    #[test]
    fn large_price() {
        assert_eq!(
            Solution::maximum_sale_items(
                vec![vec![1, 1_000_000_000], vec![1, 1_000_000_000]],
                1_000_000_000
            ),
            2
        );
    }
}
