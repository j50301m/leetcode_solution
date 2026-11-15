struct Solution {}

impl Solution {
    pub fn count_ratio_subarrays(nums: Vec<i32>, a: i32, b: i32) -> i64 {
        let n = nums.len();

        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            let num = if nums[i] % 2 == 0 {
                b as i64
            } else {
                -a as i64
            };
            prefix[i + 1] = prefix[i] + num;
        }

        let mut rank = prefix.iter().copied().collect::<Vec<i64>>();
        rank.sort_by(|left, right| right.cmp(left));
        rank.dedup();
        let mut pos_to_rank = vec![0; n + 1];
        for i in 0..n + 1 {
            let rank = rank.partition_point(|&x| x > prefix[i]);
            pos_to_rank[i] = rank + 1;
        }

        let mut bit = BIT::new(rank.len() + 1);
        let mut count = 0i64;
        for rank in pos_to_rank {
            let x = bit.query(rank);
            count += x;
            bit.insert(rank, 1);
        }

        count
    }
}

struct BIT {
    tree: Vec<i64>,
}

impl BIT {
    fn new(len: usize) -> Self {
        Self { tree: vec![0; len] }
    }
    fn query(&self, mut rank: usize) -> i64 {
        let mut count = 0;
        while rank > 0 {
            count += self.tree[rank];
            rank -= rank & rank.wrapping_neg();
        }
        count
    }

    fn insert(&mut self, mut rank: usize, count: i64) {
        while rank < self.tree.len() {
            self.tree[rank] += count;
            rank += rank & rank.wrapping_neg();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_ratio_subarrays(vec![1, 2, 1, 2], 3, 2), 7);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_ratio_subarrays(vec![2, 2, 1], 2, 1), 3);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::count_ratio_subarrays(vec![2950711, 3747997, 1250343], 1000000000, 1),
            6
        );
    }
}
