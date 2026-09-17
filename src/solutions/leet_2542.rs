use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let nums1: Vec<i64> = nums1.into_iter().map(|v| v as i64).collect();

        let mut nums2: Vec<(i64, usize)> = nums2
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (val as i64, idx))
            .collect();
        nums2.sort_by_key(|&(val, _)| Reverse(val));

        let mut heap = BinaryHeap::new();
        let mut sum = 0i64;
        for i in 0..k as usize {
            let (_, idx) = nums2[i];
            sum += nums1[idx];
            heap.push(Reverse(nums1[idx]));
        }

        let mut ans = sum * nums2[k as usize - 1].0;
        for i in k as usize..nums1.len() {
            let (min, idx) = nums2[i];
            let Reverse(top) = heap.peek().unwrap();
            if nums1[idx] < *top {
                continue;
            }
            sum = sum - top + nums1[idx];
            ans = ans.max(sum * min);
            heap.push(Reverse(nums1[idx]));
            heap.pop();
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3),
            12
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1),
            30
        );
    }

    #[test]
    fn swap_smallest_not_largest() {
        // 正解：換掉堆中「最小」的 nums1，答案 10
        assert_eq!(
            Solution::max_score(vec![1, 2, 3, 4], vec![4, 3, 2, 1], 2),
            10
        );
    }

    #[test]
    fn k_equals_len() {
        assert_eq!(Solution::max_score(vec![1, 2, 3], vec![3, 2, 1], 3), 6);
    }
}
