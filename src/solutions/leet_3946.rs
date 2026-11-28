struct Solution {}

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let n = items.len();
        let budget = budget as usize;
        let mut free_counts = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                if items[j][0] % items[i][0] == 0 {
                    free_counts[i] += 1;
                }
            }
        }

        let mut spend = vec![0; budget + 1];
        for item in 0..n {
            let prize = items[item][1] as usize;
            let get = 1 + free_counts[item];
            for money in (prize..budget + 1).rev() {
                spend[money] = spend[money].max(spend[money - prize] + get);
            }
        }

        let min_prize = items.iter().min_by_key(|&x| x[1]).unwrap()[1] as usize;
        let mut best = 0;
        for money in 0..budget + 1 {
            best = best.max(spend[money] + (budget - money) / min_prize);
        }
        best as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![6, 2], vec![2, 6], vec![3, 4]], 9),
            4
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::maximum_sale_items(
                vec![vec![2, 4], vec![3, 2], vec![4, 1], vec![6, 4], vec![12, 4]],
                8
            ),
            10
        );
    }

    // factor 相同：買一個，另一個也被送（2 能整除 2）
    #[test]
    fn same_factor() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![2, 5], vec![2, 5]], 5),
            2
        );
    }

    // 每個 price 都比 budget 大：一個都買不起
    #[test]
    fn cannot_afford() {
        assert_eq!(Solution::maximum_sale_items(vec![vec![1, 10]], 5), 0);
    }
}
