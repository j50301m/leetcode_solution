struct Solution {}

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut hit_len = vec![n + 1; n];
        let mut best = n + 1;
        let mut ans = 2 * (n + 1);

        let mut start = 0;
        let mut end = 0;
        let mut sum = 0;
        while start < n {
            if sum < target && end < n {
                sum += arr[end];
                end += 1;
                continue;
            }
            if sum == target {
                let len = end - start;
                hit_len[end - 1] = len;
                ans = ans.min(len + best);
            }
            sum -= arr[start];
            start += 1;
            best = best.min(hit_len[start - 1]);
        }
        if ans > n {
            -1
        } else {
            ans as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // LeetCode 範例 1：[3] 和 [3]，兩段長度都是 1
    #[test]
    fn case1() {
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
    }

    // LeetCode 範例 2：頭尾各一段 [7] 和 [7]，中間的 3+4 也湊得出但比較長
    #[test]
    fn case2() {
        assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
    }

    // LeetCode 範例 3：只有一段 [4,3,2,6,2,3,4] 湊得出，不足兩段
    #[test]
    fn case3_only_one() {
        assert_eq!(
            Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6),
            -1
        );
    }

    // 完全湊不出 target
    #[test]
    fn case4_none() {
        assert_eq!(Solution::min_sum_of_lengths(vec![5, 5, 4, 4, 5], 3), -1);
    }

    // 關鍵：[2,1] 和 [1,2] 都湊到 3，但共用 index 1 → 不能同時選
    #[test]
    fn case5_overlap_not_allowed() {
        assert_eq!(Solution::min_sum_of_lengths(vec![2, 1, 2], 3), -1);
    }

    // 兩個最短段 [2,1] 和 [1,2] 互相重疊，只能改配右邊的 [1,1,1]：2 + 3 = 5
    #[test]
    fn case6_must_skip_shortest() {
        assert_eq!(
            Solution::min_sum_of_lengths(vec![2, 1, 2, 9, 1, 1, 1], 3),
            5
        );
    }

    // 單一元素就等於 target，挑最靠外側的兩個
    #[test]
    fn case7_singles() {
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 1, 1, 1, 3], 3), 2);
    }

    // 剛好只夠切成兩段，整個陣列都要用上
    #[test]
    fn case8_exact_split() {
        assert_eq!(Solution::min_sum_of_lengths(vec![1, 2, 1, 2], 3), 4);
    }
}
