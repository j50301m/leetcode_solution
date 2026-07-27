struct Solution;

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len() - 1;
        let a = nums[0] * nums[1] * nums[2];
        let b = nums[n] * nums[n - 1] * nums[0];

        a.max(b)
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![1, 2, 3];
        let result = Solution::maximum_product(nums);
        assert_eq!(result, 6);
    }
}
