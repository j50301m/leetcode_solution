struct Solution {}

impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 3 {
            return 0;
        }

        nums.sort();
        (nums[n - 3] - nums[0])
            .min(nums[n - 1] - nums[2])
            .min(nums[n - 2] - nums[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::minimize_sum(vec![1, 4, 7, 8, 5]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::minimize_sum(vec![58, 42, 8, 75, 28]), 30);
    }
}
