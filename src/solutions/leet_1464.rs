struct Solution;

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() - 1;
        nums.sort();

        (nums[n] - 1) * (nums[n - 1] - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![3, 4, 5, 2];
        let result = Solution::max_product(nums);
        assert_eq!(12, result);
    }
}
