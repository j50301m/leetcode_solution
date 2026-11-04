struct Solution {}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut sorted = nums.clone();
        sorted.sort();
        sorted.dedup();

        let mut bit = BIT::new(n);
        let mut result = Vec::with_capacity(n);
        for i in (0..n).rev() {
            let rank = sorted.partition_point(|x| *x < nums[i]) + 1;
            let ans = bit.query(rank - 1);
            bit.update(rank, 1);
            result.push(ans);
        }

        result.reverse();
        result
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

    fn update(&mut self, mut rank: usize, count: i32) {
        while rank < self.tree.len() {
            self.tree[rank] += count;
            rank += Self::low_bit(rank);
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

    fn low_bit(num: usize) -> usize {
        num & !num + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    }

    #[test]
    fn duplicates_not_counted() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 2]), vec![2, 0, 1, 0]);
    }

    #[test]
    fn negatives_and_descending() {
        assert_eq!(Solution::count_smaller(vec![3, 0, -2, -5]), vec![3, 2, 1, 0]);
    }
}
