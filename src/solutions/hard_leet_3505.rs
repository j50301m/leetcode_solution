use std::collections::BTreeMap;

struct Solution {}

// 一個籃子：數字 → 出現次數，外加個數和總和
struct Bucket {
    count_of: BTreeMap<i64, usize>,
    size: usize,
    sum: i64,
}

impl Bucket {
    fn new() -> Self {
        Bucket {
            count_of: BTreeMap::new(),
            size: 0,
            sum: 0,
        }
    }

    fn insert(&mut self, value: i64) {
        *self.count_of.entry(value).or_insert(0) += 1;
        self.size += 1;
        self.sum += value;
    }

    fn remove(&mut self, value: i64) {
        let count = self.count_of.get_mut(&value).unwrap();
        *count -= 1;
        if *count == 0 {
            self.count_of.remove(&value);
        }
        self.size -= 1;
        self.sum -= value;
    }

    fn max(&self) -> Option<i64> {
        self.count_of.last_key_value().map(|(&value, _)| value)
    }

    fn min(&self) -> Option<i64> {
        self.count_of.first_key_value().map(|(&value, _)| value)
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let x = x as usize;
        let k = k as usize;
        // combinations[end] = 結尾在第 end 格的窗口的花費
        let mut combinations = vec![i64::MAX; n];
        let small_target = (x + 1) / 2; // 小籃子固定裝 ⌈x/2⌉ 個
        let mut small_bucket = Bucket::new();
        let mut big_bucket = Bucket::new();

        for end in 0..n {
            // 1. 拿出去：第 end - x 格離開窗口
            if end >= x {
                let leaving = nums[end - x] as i64;
                if leaving <= small_bucket.max().unwrap() {
                    small_bucket.remove(leaving);
                } else {
                    big_bucket.remove(leaving);
                }
            }

            // 2. 放進來：≤ 小籃子最大值就放小籃子，否則放大籃子
            //    小籃子空的時候先放大籃子，交給第 3 步把最小的搬過來
            let incoming = nums[end] as i64;
            match small_bucket.max() {
                Some(small_max) if incoming <= small_max => small_bucket.insert(incoming),
                _ => big_bucket.insert(incoming),
            }

            // 3. 調整個數
            while small_bucket.size > small_target {
                let moved = small_bucket.max().unwrap();
                small_bucket.remove(moved);
                big_bucket.insert(moved);
            }
            while small_bucket.size < small_target {
                let Some(moved) = big_bucket.min() else { break };
                big_bucket.remove(moved);
                small_bucket.insert(moved);
            }

            // 窗口滿了才算花費
            if end + 1 >= x {
                let median = small_bucket.max().unwrap();
                let small_cost = median * small_bucket.size as i64 - small_bucket.sum;
                let big_cost = big_bucket.sum - median * big_bucket.size as i64;
                combinations[end] = small_cost + big_cost;
            }
        }

        let mut dp = vec![vec![0; n + 1]; k + 1];
        for i in 1..dp.len() {
            dp[i][0] = i64::MAX;
            for j in 1..n + 1 {
                let last_cost = if j >= x {
                    if dp[i - 1][j - x] == i64::MAX {
                        i64::MAX
                    } else {
                        dp[i - 1][j - x] + combinations[j - 1]
                    }
                } else {
                    i64::MAX
                };

                let pass_curr = dp[i][j - 1];
                dp[i][j] = pass_curr.min(last_cost);
            }
        }

        dp[k][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2),
            8
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2), 3);
    }

    #[test]
    fn single_subarray() {
        // k = 1：整段 [1,2,3] 變成 2，花費 1 + 0 + 1
        assert_eq!(Solution::min_operations(vec![1, 2, 3], 3, 1), 2);
    }

    #[test]
    fn three_subarrays_already_equal() {
        // k = 3：[1,1] [2,2] [3,3] 本來就相等
        assert_eq!(Solution::min_operations(vec![1, 1, 2, 2, 3, 3], 2, 3), 0);
    }

    #[test]
    fn subarrays_must_not_overlap() {
        // index 0..2 和 1..3 都是 [0,0]，花費 0，但兩段重疊不能同時選
        // 只能選 [0,0](0..2) + [0,9](2..4) → 0 + 9
        assert_eq!(Solution::min_operations(vec![0, 0, 0, 9], 2, 2), 9);
    }

    #[test]
    fn nearest_next_window_is_not_cheapest() {
        // 窗口花費：0..2=[0,0]→0, 1..3=[0,9]→9, 2..4=[9,0]→9, 3..5=[0,0]→0
        // 最佳是 0..2 + 3..5 → 0，中間要跳過一個窗口
        assert_eq!(Solution::min_operations(vec![0, 0, 9, 0, 0], 2, 2), 0);
    }
}
