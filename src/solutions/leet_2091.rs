struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut min_idx = 0;
        let mut max_idx = 0;

        for i in 0..nums.len() {
            if nums[i] < nums[min_idx] {
                min_idx = i;
            }

            if nums[i] > nums[max_idx] {
                max_idx = i;
            }
        }

        if max_idx > min_idx {
            return (max_idx + 1)
                .min(nums.len() - min_idx)
                .min(nums.len() - max_idx + min_idx + 1) as i32;
        }

        (min_idx + 1)
            .min(nums.len() - max_idx)
            .min(nums.len() - min_idx + max_idx + 1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // min=1 在 idx 5，max=10 在 idx 1：前面刪 6 次最省
    #[test]
    fn case1() {
        assert_eq!(
            Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 7, 6, 2]),
            6
        );
    }

    // LeetCode 範例：min=-4 在 idx 1，max=19 在 idx 2
    #[test]
    fn case2() {
        assert_eq!(
            Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5]),
            3
        );
    }

    // 單一元素，min 與 max 同一個位置
    #[test]
    fn case3() {
        assert_eq!(Solution::minimum_deletions(vec![101]), 1);
    }

    // min 與 max 都靠前面，全部從前面刪
    #[test]
    fn case4() {
        assert_eq!(Solution::minimum_deletions(vec![1, 5, 2, 3, 4]), 2);
    }

    // min 與 max 都靠後面，全部從後面刪最省
    #[test]
    fn case5() {
        assert_eq!(Solution::minimum_deletions(vec![1, 2, 3, 0, 5]), 2);
    }

    // 反過來：max 在 min 前面，且兩者都靠後
    #[test]
    fn case6() {
        assert_eq!(Solution::minimum_deletions(vec![1, 2, 3, 5, 0]), 2);
    }

    // 一頭一尾，各刪一次
    #[test]
    fn case7() {
        assert_eq!(Solution::minimum_deletions(vec![9, 3, 4, 5, 1]), 2);
    }

    // 兩個元素
    #[test]
    fn case8() {
        assert_eq!(Solution::minimum_deletions(vec![7, 3]), 2);
    }
}
