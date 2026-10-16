struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let len = nums1.len() + nums2.len();
        let half_len = len / 2;

        let mut left = 0;
        let mut right = nums1.len();
        while left <= right {
            let mid = left + (right - left) / 2;

            // Cutting approach
            // nums1[..mid], nums2[..half_len -mid] -> small side
            // nums1[mid..], nums2[half_len-mid..] -> big side
            let small_1 = if mid == 0 { i32::MIN } else { nums1[mid - 1] };
            let big_1 = if mid == nums1.len() {
                i32::MAX
            } else {
                nums1[mid]
            };
            let small_2 = if half_len as i32 - mid as i32 - 1 < 0 {
                i32::MIN
            } else {
                nums2[half_len - mid - 1]
            };
            let big_2 = if (half_len as i32 - mid as i32) > nums2.len() as i32 - 1 {
                i32::MAX
            } else {
                nums2[half_len - mid]
            };

            if small_1 <= big_2 && small_2 <= big_1 {
                let left_max = small_1.max(small_2);
                let right_min = big_1.min(big_2);

                if len % 2 == 0 {
                    return (left_max as f64 + right_min as f64) / 2.0;
                } else {
                    return right_min as f64;
                }
            }

            if small_1 > big_2 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        0f64 // unreachable
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn one_side_empty() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![2, 3]), 2.5);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![]),
            3.0
        );
    }

    #[test]
    fn single_elements() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2]), 1.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![1]), 1.5);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1], vec![2, 3]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![3], vec![1, 2]),
            2.0
        );
    }

    #[test]
    fn no_overlap() {
        // nums1 全部比 nums2 小 / 大，切點會落在陣列邊界
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6]),
            3.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![4, 5, 6], vec![1, 2, 3]),
            3.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5, 6, 7]),
            4.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![6, 7], vec![1, 2, 3, 4, 5]),
            4.0
        );
    }

    #[test]
    fn interleaved_and_duplicates() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5, 7], vec![2, 4, 6]),
            4.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 1, 1], vec![1, 1]),
            1.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 2], vec![2, 2, 3]),
            2.0
        );
    }

    #[test]
    fn negatives() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![-5, -3, -1], vec![-2]),
            -2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![-1, 0], vec![0, 1]),
            0.0
        );
    }

    #[test]
    fn extreme_values() {
        // 兩個大數相加會讓 i32 溢位
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![i32::MAX], vec![i32::MAX]),
            i32::MAX as f64
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![i32::MIN], vec![i32::MAX]),
            -0.5
        );
    }
}
