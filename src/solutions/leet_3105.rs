struct Solution {}

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut longest = 1;
        let mut ascending = 1;
        let mut descending = 1;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                descending += 1;
                ascending = 1;
                longest = longest.max(descending);
            } else if nums[i] > nums[i - 1] {
                ascending += 1;
                descending = 1;
                longest = longest.max(ascending);
            } else {
                ascending = 1;
                descending = 1;
            }
        }

        longest
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    }
}
