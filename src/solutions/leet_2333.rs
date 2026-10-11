struct Solution {}

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut diff = vec![0i64; 100001];
        let mut k = k1 as i64 + k2 as i64;
        for i in 0..nums1.len() {
            let idx = (nums1[i] - nums2[i]).abs() as i64;
            diff[idx as usize] += 1;
        }

        for num in (1..=100000).rev() {
            let cnt = diff[num];
            if k == 0 || num == 0 {
                break;
            }

            if cnt == 0 {
                continue;
            }

            if cnt > k {
                diff[num] -= k;
                diff[num - 1] += k;
                k = 0;
            } else {
                diff[num] = 0;
                diff[num - 1] += cnt;
                k -= cnt;
            }
        }

        diff.iter()
            .enumerate()
            .fold(0, |acc, (num, &cnt)| acc + (num * num) as i64 * cnt)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        // k = 0，什麼都不能改
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0),
            579
        );
    }

    #[test]
    fn case2() {
        // diffs = [4, 4, 4, 3]，k = 2 → [3, 3, 4, 3]... 應該改最大的
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1),
            43
        );
    }

    #[test]
    fn case3() {
        // 同一層有 2 個 diff=3，但 k 只有 1 → [2, 3]
        assert_eq!(
            Solution::min_sum_square_diff(vec![0, 0], vec![3, 3], 1, 0),
            13
        );
    }

    #[test]
    fn case4() {
        // k 剛好把所有 diff 歸零
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3], vec![2, 3, 4], 2, 1),
            0
        );
    }

    #[test]
    fn case5() {
        // k 用不完
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3], vec![2, 3, 4], 5, 5),
            0
        );
    }

    #[test]
    fn case6() {
        // k1 + k2 = 2e9，超過 i32::MAX
        assert_eq!(
            Solution::min_sum_square_diff(vec![0], vec![100000], 1_000_000_000, 1_000_000_000),
            0
        );
    }
}
