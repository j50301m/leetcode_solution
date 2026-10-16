struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[right] {
                left = mid + 1;
            } else if nums[mid] < nums[right] {
                right = mid;
            } else if nums[mid] == nums[right] {
                right -= 1;
            }
        }

        nums[left]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::find_min(vec![7]), 7);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::find_min(vec![3, 3, 3, 3]), 3);
    }

    #[test]
    fn min_on_left_of_equal_mid() {
        assert_eq!(Solution::find_min(vec![1, 0, 1, 1, 1]), 0);
    }

    #[test]
    fn min_on_right_of_equal_mid() {
        assert_eq!(Solution::find_min(vec![1, 1, 1, 0, 1]), 0);
    }

    #[test]
    fn not_rotated_with_duplicates() {
        assert_eq!(Solution::find_min(vec![1, 1, 2, 2, 3]), 1);
    }
}
