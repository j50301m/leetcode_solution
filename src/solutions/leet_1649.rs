struct Solution {}

const MOD: i32 = 1000000007;
impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let n = instructions.len();

        let mut sorted = instructions.clone();
        sorted.sort();
        sorted.dedup();

        let mut ranks = vec![0usize; 100001];
        let mut curr_rank = 0;
        for i in 0..sorted.len() {
            let idx = sorted[i] as usize;
            curr_rank += 1;
            ranks[idx] = curr_rank;
        }

        let mut bit = BIT::new(n);
        let mut sum = 0;
        for i in 0..n {
            let idx = instructions[i] as usize;
            let rank = ranks[idx];
            let smaller_count = bit.query(rank - 1);
            let biger_count = i as i32 - bit.query(rank);
            sum = (sum + smaller_count.min(biger_count)) % MOD;

            bit.update(rank, 1);
        }

        sum
    }
}

struct BIT {
    tree: Vec<i32>,
}

impl BIT {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn query(&self, mut rank: usize) -> i32 {
        let mut count = 0;
        while rank > 0 {
            count += self.tree[rank];
            rank -= Self::low_bit(rank);
        }
        count
    }

    fn update(&mut self, mut rank: usize, val: i32) {
        let n = self.tree.len();
        while rank < n {
            self.tree[rank] += val;
            rank += Self::low_bit(rank);
        }
    }

    fn low_bit(n: usize) -> usize {
        n & (!n + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::create_sorted_array(vec![1, 5, 6, 2]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]), 3);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]),
            4
        );
    }

    #[test]
    fn single_element_costs_nothing() {
        assert_eq!(Solution::create_sorted_array(vec![7]), 0);
    }

    #[test]
    fn equal_numbers_are_not_counted() {
        // 一樣大的數字既不算「比較小」也不算「比較大」
        assert_eq!(Solution::create_sorted_array(vec![2, 2, 2]), 0);
    }

    #[test]
    fn large_values_with_few_elements() {
        // 數字很大但只有 3 個：[100000] → 0，[100000, 1] → 0，再插 50000 → min(1, 1) = 1
        assert_eq!(Solution::create_sorted_array(vec![100000, 1, 50000]), 1);
    }

    #[test]
    fn long_descending_input_costs_nothing() {
        // 由大到小插入，每次比它大的有 i 個、比它小的有 0 個，所以每次花費都是 0
        let instructions: Vec<i32> = (1..=100000).rev().collect();
        assert_eq!(Solution::create_sorted_array(instructions), 0);
    }
}
