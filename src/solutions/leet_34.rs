struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len();

        let mut start = -1;
        let mut end = -1;
        while left < right {
            let mid = left + (right - left) / 2;

            if target == nums[mid] {
                start = mid as i32;
                right = mid;
            } else if target > nums[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left = start as usize;
        right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;

            if target == nums[mid] {
                end = mid as i32;
                left = mid + 1;
            } else if target > nums[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        vec![start, end]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }
}
