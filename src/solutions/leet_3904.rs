struct Solution {}

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut mins = vec![nums[n - 1]; n];
        for i in (0..n - 1).rev() {
            mins[i] = if nums[i] < mins[i + 1] {
                nums[i]
            } else {
                mins[i + 1]
            };
        }

        let mut max = nums[0];
        for i in 0..n {
            max = if nums[i] > max { nums[i] } else { max };
            if max - mins[i] <= k {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 官方 example：[5,0,1,4], k=3 -> 分數 5,5,4,1，第一個 <=3 的是 i=3
    #[test]
    fn case1() {
        assert_eq!(Solution::first_stable_index(vec![5, 0, 1, 4], 3), 3);
    }

    // 同一組數字，k 放寬到 4 -> 答案往前挪到 i=2（分數 4）
    #[test]
    fn case2() {
        assert_eq!(Solution::first_stable_index(vec![5, 0, 1, 4], 4), 2);
    }

    // k 再放寬到 5 -> i=0 就成立
    #[test]
    fn case3() {
        assert_eq!(Solution::first_stable_index(vec![5, 0, 1, 4], 5), 0);
    }

    // k=0 全部都不成立 -> -1（分數最小的 i=3 也還有 1）
    #[test]
    fn case4() {
        assert_eq!(Solution::first_stable_index(vec![5, 0, 1, 4], 0), -1);
        assert_eq!(Solution::first_stable_index(vec![1, 0], 0), -1);
    }

    // n == 1：prefixMax 和 suffixMin 都是自己，分數必為 0
    // -> 抓 (0..n-1) 在 n=1 時的 usize 溢位 / min 陣列開太小
    #[test]
    fn case5() {
        assert_eq!(Solution::first_stable_index(vec![7], 0), 0);
        assert_eq!(Solution::first_stable_index(vec![1_000_000_000], 0), 0);
    }

    // 全部相同 + k=0 -> i=0 直接成立
    #[test]
    fn case6() {
        assert_eq!(Solution::first_stable_index(vec![3, 3, 3], 0), 0);
    }

    // 遞減陣列：prefixMax 永遠是 nums[0]，suffixMin 永遠是 nums[n-1]，每格分數都一樣
    // -> 抓「用 nums[i] 當左邊的最大值」的錯誤解（那樣 i=1 會算成 8-7=1 而誤回 1）
    #[test]
    fn case7() {
        assert_eq!(Solution::first_stable_index(vec![9, 8, 7], 1), -1);
        assert_eq!(Solution::first_stable_index(vec![9, 8, 7], 2), 0);
    }

    // 最小值躲在最後面 -> 抓「用 nums[i] 當右邊的最小值」的錯誤解
    // 錯的版本在 i=0 會算成 4-4=0 而誤回 0，正解是 4-0=4 > 1 -> -1
    #[test]
    fn case8() {
        assert_eq!(Solution::first_stable_index(vec![4, 5, 0], 1), -1);
    }

    // 分數會隨 i 上下跳（suffixMin 遞增、prefixMax 也遞增，不是單調的）
    // [10,1,2,3] 分數 9,9,8,7 -> k=8 時第一個成立的是 i=2
    #[test]
    fn case9() {
        assert_eq!(Solution::first_stable_index(vec![10, 1, 2, 3], 8), 2);
        assert_eq!(Solution::first_stable_index(vec![10, 1, 2, 3], 7), 3);
        assert_eq!(Solution::first_stable_index(vec![10, 1, 2, 3], 6), -1);
    }

    // 有重複值 -> prefixMax / suffixMin 的比較用 > 還是 >= 都不該影響結果
    #[test]
    fn case10() {
        assert_eq!(Solution::first_stable_index(vec![2, 2, 2, 1], 1), 0);
        assert_eq!(Solution::first_stable_index(vec![2, 2, 2, 1], 0), -1);
    }

    // 值域上界 1e9，差值剛好 1e9 -> 抓 i32 邊界（1e9 沒溢位，但 k 也可能是 1e9）
    #[test]
    fn case11() {
        assert_eq!(
            Solution::first_stable_index(vec![1_000_000_000, 0], 1_000_000_000),
            0
        );
        assert_eq!(
            Solution::first_stable_index(vec![1_000_000_000, 0], 999_999_999),
            -1
        );
    }

    // n = 100 滿載，遞減 99..0：每格分數都是 99
    #[test]
    fn case12() {
        let nums: Vec<i32> = (0..100).rev().collect();
        assert_eq!(Solution::first_stable_index(nums.clone(), 99), 0);
        assert_eq!(Solution::first_stable_index(nums, 98), -1);
    }
}
