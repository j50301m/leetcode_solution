struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let total: i32 = nums.iter().sum();
        let total_diff = total - x;
        if total_diff == 0 {
            return nums.len() as i32;
        }
        if total_diff < 0 {
            return -1;
        }

        let (mut left, mut right) = (0usize, 0usize);
        let mut max_len = 0;
        let mut sum = 0;
        while right < nums.len() || left < right {
            if sum == total_diff {
                max_len = max_len.max(right - left);
            }

            if sum < total_diff && right < nums.len() {
                sum += nums[right];
                right += 1;
            } else {
                sum -= nums[left];
                left += 1;
            }
        }

        if max_len == 0 {
            return -1;
        }
        (nums.len() - max_len) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    }

    // 只從左邊拿（suffix 取 0 個）
    #[test]
    fn prefix_only() {
        assert_eq!(Solution::min_operations(vec![2, 5, 5], 2), 1);
    }

    // 只從右邊拿（prefix 取 0 個）
    #[test]
    fn suffix_only() {
        assert_eq!(Solution::min_operations(vec![5, 5, 2], 2), 1);
    }

    // 整個陣列都要拿完
    #[test]
    fn take_all() {
        assert_eq!(Solution::min_operations(vec![1, 1], 2), 2);
    }

    // x 大於總和：prefix 和 suffix 不能重疊同一個元素
    #[test]
    fn no_overlap() {
        assert_eq!(Solution::min_operations(vec![1, 1], 3), -1);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::min_operations(vec![1], 1), 1);
    }
}
