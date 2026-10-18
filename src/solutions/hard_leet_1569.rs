struct Solution {}

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_ways_1(nums: Vec<i32>) -> i32 {
        let length = nums.len();

        // 帕斯卡三角形：binomial[row][col] = C(row, col)
        // 每格 = 左上 + 正上，只用加法，所以可以邊加邊取模
        let mut binomial = vec![vec![0_i64; length + 1]; length + 1];
        for row in 0..=length {
            binomial[row][0] = 1;
            for col in 1..=row {
                binomial[row][col] = (binomial[row - 1][col - 1] + binomial[row - 1][col]) % MOD;
            }
        }

        // ways 包含原本那個排列，題目問「別的」排列，所以減 1
        // 先加 MOD 再減，避免 ways 取模後剛好是 0 時變成負數
        let ways = Self::dfs(&nums, &binomial);
        ((ways + MOD - 1) % MOD) as i32
    }

    // 回傳：有幾種排列可以長出和 sequence 同一棵 BST（包含 sequence 自己）
    fn dfs1(sequence: &[i32], binomial: &Vec<Vec<i64>>) -> i64 {
        // 0、1、2 個數都只有一種排法（2 個數時，根一定要排在前面）
        if sequence.len() <= 2 {
            return 1;
        }

        // 第一個數一定是根
        let root = sequence[0];

        // 照原本的順序把剩下的數分成左群和右群
        // 左群內部的順序決定左子樹的形狀，所以不能打亂
        let left_group: Vec<i32> = sequence[1..]
            .iter()
            .copied()
            .filter(|&value| value < root)
            .collect();
        let right_group: Vec<i32> = sequence[1..]
            .iter()
            .copied()
            .filter(|&value| value > root)
            .collect();

        // 根後面有 len - 1 個位置，挑 left_group.len() 個給左群：C(len - 1, 左群大小)
        let interleavings = binomial[sequence.len() - 1][left_group.len()];

        // 左右子樹各自也是同一個問題，遞迴下去
        let left_ways = Self::dfs(&left_group, binomial);
        let right_ways = Self::dfs(&right_group, binomial);

        // 每乘一次就取模，三個數直接連乘會溢位
        interleavings * left_ways % MOD * right_ways % MOD
    }

    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut binomial = vec![vec![1; n + 1]; n + 1];

        for i in 2..n {
            for j in 1..i {
                binomial[i][j] = (binomial[i - 1][j - 1] + binomial[i - 1][j]) % MOD;
            }
        }

        ((Self::dfs(&nums, &binomial) + MOD - 1) % MOD) as i32
    }

    fn dfs(sequence: &[i32], binomial: &Vec<Vec<i64>>) -> i64 {
        if sequence.len() < 3 {
            return 1;
        }

        let root = sequence[0];

        let left_groups: Vec<i32> = sequence[1..]
            .iter()
            .copied()
            .filter(|&x| x < root)
            .collect();

        let right_groups: Vec<i32> = sequence[1..]
            .iter()
            .copied()
            .filter(|&x| x > root)
            .collect();

        let interleavings = binomial[sequence.len() - 1][left_groups.len()];
        let left_ways = Self::dfs(&left_groups, binomial);
        let right_ways = Self::dfs(&right_groups, binomial);
        interleavings * left_ways % MOD * right_ways % MOD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::num_of_ways(vec![2, 1, 3]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::num_of_ways(vec![3, 4, 5, 1, 2]), 5);
    }

    #[test]
    fn sorted_has_no_other_way() {
        assert_eq!(Solution::num_of_ways(vec![1, 2, 3]), 0);
    }

    #[test]
    fn large_sorted_does_not_overflow_stack() {
        let nums: Vec<i32> = (1..=1000).collect();
        assert_eq!(Solution::num_of_ways(nums), 0);
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::num_of_ways(vec![
                10, 23, 12, 18, 4, 29, 2, 8, 41, 31, 25, 21, 14, 35, 26, 5, 19, 43, 22, 37, 9, 20,
                44, 28, 1, 39, 30, 38, 36, 6, 13, 16, 27, 17, 34, 7, 15, 3, 11, 24, 42, 33, 40, 32
            ]),
            182440977
        );
    }
}
