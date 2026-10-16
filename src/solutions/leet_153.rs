struct Solution {}

impl Solution {
    pub fn find_min1(nums: Vec<i32>) -> i32 {
        let last_ele = nums.last().unwrap();
        let pos = nums.partition_point(|&x| x > *last_ele);
        nums[pos]
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < nums[right] {
                right = mid;
            } else {
                left = mid + 1;
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
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }
}
